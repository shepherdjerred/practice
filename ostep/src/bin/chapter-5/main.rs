use std::ffi::CString;
use nix::{libc, sys::wait::waitpid, unistd::{fork, ForkResult, write}};
use nix::libc::{O_CREAT, O_RDWR, open};

fn main() {
    let mut x = 1;
    x = 2;
    unsafe {
        let handle = open(CString::new("/tmp/my-file").unwrap_unchecked().as_ptr(), O_CREAT & O_RDWR);
        println!("{}", handle);
        write(handle, "hello world".as_bytes());
    }
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            waitpid(child, None).unwrap();
            x += 1;
            println!("{}", x);
        }
        Ok(ForkResult::Child) => {
            write(libc::STDOUT_FILENO, "I'm a new child process
".as_bytes()).ok();
            x += 2;
            println!("{}", x);
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }
}
