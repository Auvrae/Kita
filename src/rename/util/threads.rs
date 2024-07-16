use std::thread::{park_timeout, sleep, spawn, JoinHandle};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::{fs, io, vec};

use super::dir;
use super::hash;
use super::super::mods;
use super::processing::file_processing;
use super::super::app;

fn timeout(time: Duration) {
    park_timeout(time);
}

fn rename_file(original_path: String, renamed_path: String) -> Result<(), io::Error> {
    fs::rename(original_path, renamed_path)
}

pub fn thread(gui: &mut app::WindowMain, func: ThreadFunction) -> JoinHandle<()> {
    let progress = Arc::clone(&gui.thread_storage.progress);
    let hashes = Arc::clone(&gui.thread_storage.hashes);
    let errors = Arc::clone(&gui.thread_storage.errors);
    let state = Arc::clone(&gui.thread_storage.state);


    match func {
        ThreadFunction::Hash(algorithm, paths, endianness) => {
            spawn(move ||{
                struct Thread {
                    thread_number: u32,
                    handler: Option<JoinHandle<()>>,
                    hash: Arc<Mutex<(String, usize, usize)>>
                }
                let progress_slice: f32 = (1.0 / paths.len() as f32) * 100.0;
    
                // Update Internal State
                *state.lock().unwrap() = ThreadState::Saving;
                let mut internal_hashes: Vec<(String, usize, usize)> = vec![];
                for _ in 1..=paths.len() {internal_hashes.push((String::new(), 0, 0))};
                
                // Make thread groups
                let cpu_threads = 4 as usize; //std::thread::available_parallelism().unwrap().get();
                let mut paths = paths;
                paths.reverse();
                let groups: u32 = (paths.len() as f32 / cpu_threads as f32).ceil() as u32;
                let mut thread_counter: u32 = 0;
    
                // Thread Groups (A thread for each core)
                for _ in 1..=groups {
                    let mut threads: Vec<Thread> = vec![];
    
                    // Per Core Threads
                    for _ in 1..=cpu_threads {
                        match paths.pop() {
                            Some(path) => {
                                match algorithm {
                                    HashType::CRC32 => {
                                        let mut thread = Thread {
                                            thread_number: thread_counter.to_owned(),
                                            handler: None,
                                            hash: Arc::new(Mutex::new((String::new(), 0, 0)))
                                        };
    
                                        let thread_hash = Arc::clone(&thread.hash);
                                        thread.handler = Some(spawn(move || {
                                            let hash = hash::hash_file(path.0.to_owned(), &hash::HashType::CRC32, endianness);
                                            *thread_hash.lock().unwrap() = (hash, path.1, path.2);
                                        }));
                                        threads.push(thread)
                                    },
                                    HashType::MD5 => {
                                        let mut thread = Thread {
                                            thread_number: thread_counter.to_owned(),
                                            handler: None,
                                            hash: Arc::new(Mutex::new((String::new(), 0, 0)))
                                        };
    
                                        let thread_hash = Arc::clone(&thread.hash);
                                        thread.handler = Some(spawn(move || {
                                            let hash = hash::hash_file(path.0.to_owned(), &hash::HashType::MD5, endianness);
                                            *thread_hash.lock().unwrap() = (hash, path.1, path.2);
                                        }));
                                        threads.push(thread)
                                    },
                                    HashType::Sha1 => {
                                        let mut thread = Thread {
                                            thread_number: thread_counter.to_owned(),
                                            handler: None,
                                            hash: Arc::new(Mutex::new((String::new(), 0, 0)))
                                        };
    
                                        let thread_hash = Arc::clone(&thread.hash);
                                        thread.handler = Some(spawn(move || {
                                            let hash = hash::hash_file(path.0.to_owned(), &hash::HashType::Sha1, endianness);
                                            *thread_hash.lock().unwrap() = (hash, path.1, path.2);
                                        }));
                                        threads.push(thread)
                                    },
                                    HashType::Sha256 => {
                                        let mut thread = Thread {
                                            thread_number: thread_counter.to_owned(),
                                            handler: None,
                                            hash: Arc::new(Mutex::new((String::new(), 0, 0)))
                                        };
    
                                        let thread_hash = Arc::clone(&thread.hash);
                                        thread.handler = Some(spawn(move || {
                                            let hash = hash::hash_file(path.0.to_owned(), &hash::HashType::Sha256, endianness);
                                            *thread_hash.lock().unwrap() = (hash, path.1, path.2);
                                        }));
                                        threads.push(thread)
                                    }
                                }
                            },
                            None => {}
                        }
                        thread_counter += 1;
                    }
                    let mut threads_alive: bool = true;
                    let mut threads_dead: Vec<usize> = vec![];
                    while threads_alive {
                        for index in 0..threads.len().to_owned() {
                            let handler = threads[index].handler.take().unwrap();
                            if handler.is_finished() {
                                handler.join().unwrap();
                                internal_hashes.remove(threads[index].thread_number as usize);
                                internal_hashes.insert(threads[index].thread_number as usize, threads[index].hash.lock().unwrap().to_owned());
                                *progress.lock().unwrap() += progress_slice;
                                threads_dead.push(index);
                            } else {
                                threads[index].handler = Some(handler);
                            }
                        }
                        threads_dead.sort_by(|a, b | b.cmp(a));
                        for (_, index) in threads_dead.iter().enumerate() {
                            threads.remove(index.to_owned());
                        }
                        threads_dead = vec![];
                        if threads.len() == 0 { threads_alive = false }
                    }
                }
                *hashes.lock().unwrap() = internal_hashes;
                *state.lock().unwrap() = ThreadState::Completed;
            }) 
        },
        
        ThreadFunction::SaveUndoRedo(edit) => {
            spawn(move || {
                let progress_slice: f32 = (1.0 / (edit.items.len() as f32 - 1.0) as f32) * 100.0;
    
                // Update Internal State
                *state.lock().unwrap() = ThreadState::Saving;
                let mut errored: bool = false;
                let mut errs: Vec<String> = vec![];
                // Commit Changes
                for item in &edit.items {
                    match rename_file(item.path_original.to_owned(), item.path_edited.to_owned()) {
                        Ok(_) => {
                            *progress.lock().unwrap() += progress_slice;
                        },
                        Err(err) => {
                            errored = true;
                            println!("{}", err.to_string());
                            errs.push(err.to_string());
                            *state.lock().unwrap() = ThreadState::Errored;
                        }
                    }
                    timeout(Duration::from_millis(1));
                }
                if errored {
                    *errors.lock().unwrap() = errs.to_owned();
                    std::process::exit(1);
                }
                *state.lock().unwrap() = ThreadState::Completed;
            })
        },
        
        ThreadFunction::StringProcessing(refresh) => {
            let kill_sig = Arc::clone(&gui.modifier_thread_storage.kill_sig_string_processor);
            let modifiers = Arc::clone(&gui.modifier_thread_storage.modifiers);
            let modifier_order = Arc::clone(&gui.modifier_thread_storage.modifier_order);
            let eddited_files = Arc::clone(&gui.modifier_thread_storage.eddited_files);
            let raw_files = Arc::clone(&gui.modifier_thread_storage.raw_files);
            let errors = Arc::clone(&gui.modifier_thread_storage.errors);
            let state = Arc::clone(&gui.modifier_thread_storage.state);
            let frame_time = Arc::clone(&gui.modifier_thread_storage.thread_calc_time);
            spawn(move || {
                let per_second: u128 = 1000 / refresh as u128;
                *state.lock().unwrap() = ThreadState::Working;
                loop {
                    // Check kill signal
                    if *kill_sig.lock().unwrap() == true { 
                        *state.lock().unwrap() = ThreadState::Dead;
                        break; 
                    };

                    let start = Instant::now();

                    // Check if Arc is empty for both.
                    let mut files = raw_files.lock().unwrap().clone();
                    let mut mods = modifiers.lock().unwrap().clone();
                    let mut mod_order = modifier_order.lock().unwrap().clone();
                    if files.is_none() || mods.is_none() || mod_order.is_none() {
                        if start.clone().elapsed().as_millis() < per_second as u128 {
                            sleep(Duration::from_millis((per_second - start.clone().elapsed().as_millis()) as u64));
                        }
                        continue; 
                    }
                    else { 
                        *raw_files.lock().unwrap() = None;
                        *modifiers.lock().unwrap() = None;
                        *modifier_order.lock().unwrap() = None;
                    };
                    let proto_files = files.take().unwrap();
                    let mut mods = mods.take().unwrap();
                    let mod_order = mod_order.take().unwrap();

                    // Edit all the proto-files with the modifiers
                    let mut completed_edits: Vec<(Vec<(String, usize, Option<String>)>, Vec<(String, usize, Option<String>)>)> = vec![];
                    let mut completed_errors: Vec<(Vec<ModifierThreadError>, Vec<ModifierThreadError>)> = vec![];
                    for (index, (folders, files)) in proto_files.iter().enumerate() {
                        let folders_edits = file_processing::process(index, &mut mods, folders.to_owned(), mod_order.clone(), true);
                        let files_edits = file_processing::process(index, &mut mods, files.to_owned(), mod_order.clone(), false);
                        completed_edits.push((folders_edits.0, files_edits.0));
                        completed_errors.push((folders_edits.1, files_edits.1));
                    };

                    // See how long we need to sleep
                    let end = start.clone().elapsed();
                    *frame_time.lock().unwrap() = end.as_millis() as u32;
                    //println!("Thread processor took {}ms", end.as_millis());
                    if end.as_millis() < per_second as u128 {
                        sleep(Duration::from_millis((per_second - end.as_millis()) as u64));
                    };
                    // Return editted_files and errors
                    *eddited_files.lock().unwrap() = Some(completed_edits);
                    *errors.lock().unwrap() = Some(completed_errors);
                }
            })
        },
    }
}

#[derive(Clone)]
// Modifier Thread
pub struct ModifierThreadStorage {
    pub kill_sig_string_processor: Arc<Mutex<bool>>,
    pub modifiers: Arc<Mutex<Option<mods::Modifiers>>>,
    pub modifier_order: Arc<Mutex<Option<Vec<mods::ModsOrder>>>>,
    pub eddited_files: Arc<Mutex<Option<Vec<(Vec<(String, usize, Option<String>)>, Vec<(String, usize, Option<String>)>)>>>>,
    pub raw_files: Arc<Mutex<Option<Vec<(Vec<(String, usize, Option<String>)>, Vec<(String, usize, Option<String>)>)>>>>,
    pub errors: Arc<Mutex<Option<Vec<(Vec<ModifierThreadError>, Vec<ModifierThreadError>)>>>>,
    pub state: Arc<Mutex<ThreadState>>,
    pub thread_calc_time: Arc<Mutex<u32>>
}

#[derive(Clone)]
// Storage for Threads
pub struct ThreadStorage {
    pub progress: Arc<Mutex<f32>>,
    pub hashes: Arc<Mutex<Vec<(String, usize, usize)>>>,
    pub errors: Arc<Mutex<Vec<String>>>,
    pub state: Arc<Mutex<ThreadState>>,
}

impl Default for ThreadStorage {
    fn default() -> Self {
        Self {
            progress: Arc::new(Mutex::new(f32::default())),
            hashes: Arc::new(Mutex::new(Vec::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(Mutex::new(ThreadState::None)),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Copy, Serialize, Deserialize)]
pub enum HashMode {
    None,
    Prefix,
    Suffix,
    File
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum HashType {
    CRC32,
    MD5,
    Sha1,
    Sha256
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState {
    None,
    Saving,
    Working,
    Completed,
    Errored,
    Dead
}

#[derive(Clone)]
pub enum ThreadFunction {
    Hash(HashType, Vec<(String, usize, usize)>, Endianness),
    SaveUndoRedo(dir::Edit),
    StringProcessing(u8)
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum Endianness {
    BigEndian,
    _LittleEndian
}

#[derive(Clone, Debug)]
pub enum ModifierThreadError {
    /// Vec<File Index>
    DuplicateFileName(Vec<usize>), 
    /// Vec<(File Index, Length)>
    LengthLimitFileName(Vec<(usize, u32)>), 
    /// Vec<(File Index, Char Index, Invalid Char)>
    InvalidChar(Vec<(usize, char)>),
    /// Vec<(File Indexx, Invalid String)>
    InvalidFileName(Vec<(usize, String)>)
}