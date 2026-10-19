#![cfg_attr(not(any(test, doctest)), no_std)]

extern crate alloc;

use {
    alloc::boxed::Box,
    anyhow::Result,
    core::{
        cell::Ref,
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
    },
    crankstart::{CrankstartAPI, Game, crankstart_game, eprintln, util::singleton::Singleton},
    euclid::{Length, Scale},
};

struct Frame;
struct Second;

struct FileDemo {
    max_work_seconds: Length<f32, Second>,
    future: Option<Pin<Box<dyn Future<Output = Result<()>>>>>,
}

impl FileDemo {
    async fn run() -> Result<()> {
        Ok(())
    }
}

impl Game for FileDemo {
    fn new() -> Result<Self> {
        const FRAMES: Length<f32, Frame> = Length::<f32, Frame>::new(1.0_f32);

        // We should be able to get this from the API, but it doesn't look like the current bindgen
        // has a function for that? I'm seeing reports online that one exists.
        const FRAMES_PER_SECOND: Scale<f32, Second, Frame> = Scale::new(30.0_f32);
        const WORK_LOAD_PER_FRAME: f32 = 0.75_f32;

        let max_work_seconds: Length<f32, Second> =
            FRAMES * FRAMES_PER_SECOND.inverse() * WORK_LOAD_PER_FRAME;
        let future: Option<Pin<Box<dyn Future<Output = Result<()>>>>> = Some(Box::pin(Self::run()));

        Ok(Self {
            max_work_seconds,
            future,
        })
    }

    fn update(&mut self) -> Result<bool> {
        if let Some(future) = &mut self.future {
            let crankstart: Ref<CrankstartAPI> = CrankstartAPI::get();

            crankstart.system.reset_elapsed_time();

            let mut context: Context = Context::from_waker(Waker::noop());
            let mut future_is_ready;

            while {
                future_is_ready = match future.as_mut().poll(&mut context) {
                    Poll::Ready(result) => {
                        if let Err(e) = result {
                            eprintln!("`FileDemo::run` returned error:\n{e:?}");
                        }

                        true
                    }
                    Poll::Pending => false,
                };

                !future_is_ready
                    && Length::<f32, Second>::new(crankstart.system.get_elapsed_time())
                        < self.max_work_seconds
            } {}

            if future_is_ready {
                self.future = None;
            }
        }

        Ok(false)
    }
}

crankstart_game!(FileDemo);

#[cfg(test)]
mod tests {
    use {
        arrayvec::ArrayVec,
        crankstart::util::{crank::Target, string::TempString},
        semver::Version,
        std::{
            fs::{File, create_dir, create_dir_all},
            io::Write,
            ops::RangeInclusive,
        },
    };

    type TokenArrayVec = ArrayVec<char, { SimpleFileGenerator::TOKENS_CAPACITY }>;

    #[derive(Default)]
    struct SimpleFileGenerator {
        tokens: TokenArrayVec,
        path: TempString,
    }

    impl SimpleFileGenerator {
        const SIMPLE_FILE_ROOT: &'static str = "./assets/simple";
        const TOKEN_DELIMITER: char = '/';
        const TEXT_FILE_EXTENSION: &'static str = ".txt";
        const TOKEN_RANGE: RangeInclusive<char> = 'a'..='c';
        const TOKENS_CAPACITY: usize = 3_usize;

        fn generate_simple_files(&mut self) {
            create_dir_all(Self::SIMPLE_FILE_ROOT).ok();

            self.tokens.clear();
            self.path.clear();
            self.generate_simple_files_internal();
        }

        fn generate_simple_files_internal(&mut self) {
            if self.tokens.is_full() {
                if let Ok(mut file) = File::create(self.path.as_str()) {
                    file.write(self.path.as_str().as_bytes()).ok();
                }
            } else {
                for token in Self::TOKEN_RANGE {
                    self.push_token(token);

                    if !self.tokens.is_full() {
                        create_dir(self.path.as_str()).ok();
                    }

                    self.generate_simple_files_internal();
                    self.pop_token();
                }
            }
        }

        fn push_token(&mut self, token: char) {
            assert!(!self.tokens.is_full());

            self.tokens.push(token);
            self.update_path();
        }

        fn pop_token(&mut self) {
            assert!(!self.tokens.is_empty());

            self.tokens.pop();
            self.update_path();
        }

        fn update_path(&mut self) {
            self.path.clear();
            self.path.push_str(Self::SIMPLE_FILE_ROOT);

            for token in self.tokens.iter().copied() {
                self.path.push(Self::TOKEN_DELIMITER);
                self.path.push(token);
            }

            if self.tokens.is_full() {
                self.path.push_str(Self::TEXT_FILE_EXTENSION);
            }
        }
    }

    #[test]
    fn generate_simple_files() {
        SimpleFileGenerator::default().generate_simple_files();
    }

    #[test]
    fn generate_crank() {
        Target::try_new(
            "file",
            Version::new(0, 1, 0),
            "ClydeHobart",
            "dev.clydehobart.crankstart.file",
            "./assets",
        )
        .unwrap()
        .try_write(".")
        .unwrap();
    }
}
