use std::fs::File;
use std::io::{Write, Error};
use std::process::{Command, Stdio};

fn main() {
const RUSTUP_CMD: &str = "rustup";
const RUSTUP_DOC_ARG: &str = "docs";
const RUSTUP_PATH_ARG: &str = "--path";
const CD_CMD: &str = "cd -verbose"; //reread manpage

    fn spawn_python_webserver (docoption: &str) {
        let mut path_to_webserver_root = Command::new(RUSTUPCMD)
            .args(&[DOCARG, PATHARG])
            .arg("--".push_str(docoption))//for now just take a &str later do enum match
            //secound problem push_str may not be the best oh well
            .output()?;
    /*
        enum docoptions {
            alloc,
            book,
            cargo,
            clippy,
            core,
            edition_guide,
            embedded-book,
            error-codes,
            nomicoin,
            proc_macro,
            reference,
            rust-by-example,
            rustc,
            rustdoc,
            std,
            style-guide,
            test,
            unstable-book,
        }
    */
    //future set toolchain as of now i just want the path to all the books and docs

        //execute a bash cmd and catch error
        if !path_to_webserver_root.status.success() {
            error_chain::bail!("Command executed with failing error code");
        } 
   
        path_to_webserver_root = ;
   /*still need to remove index at the end of the path to the docs then cd 
    so trim, but not doing a trim idk reverse search,
    invert string the and remove charaters until first '\'
    */
        let change_dir = Command::new(CD_CMD).arg(path_to_webserver_root); //change dir
    
        //execute a bash cmd and catch error
        if !change_dir.status.success() {
            error_chain::bail!("Command executed with failing error code");
        } 
    
        let mut child = Command::new("python3 -m http.server -b 127.0.0.43 8080").stdin(Stdio::piped()) //same amount as what ever section i want
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;    
    }
}

/*
    so i can spawn a webserver to offer the docs that are in userland and use firefox to open them up
    not so sure how useland interacts with other apps and the address space, so the user should
    just know that it might collide with something existing or clobber it. i dont remember how
    andoid does the network stack and permission on a per app basis. maybe its doing some
    nat stuff and the blobbering just works or its stricter and it will not allow a fd to open
    
    on the more technical side of the linux box. i can write the python stout to a log file 
    on the the inside. but with rust running this the ps tree should look similar to this
        userland - bash
                    |-rust binary I
                        |-bash
                            |-rustup(runs and dies)
                            |-cd(runs and dies)
                            |-python webserver(runs and writes to a log file) I, II
        I. the rust program can hook the pyhton std out to a log file 
        II. so bash is nested inside bash five times not bash inside a bash inside onestla
            but bash and five nodes of bash connecting to it
            and each nested bash(nb) has its own python webserver running
            each need a diffent ip and socket i think
            and each will have two diffent fd associtated with it one for network
            another for writing to a log for the webserver
            rust binary should have its own log for the occastration of each webserver
            and child fd i think i for get how bash does that, and it might be a 
            language specific thing, like rust might as bash/os to spawn a new bash
            a diffent way, may not be nested, could be a fork
            which make make running something like gdb or lldb complicated with
            memory mapping and process trees and spaces
            and to keep in mind this is all inside userland an emulation shim/layer
            for android to run a linux shell inside of it
            so most likely debugging will be done with lldb cause the primary applicatoin
            is an app, but gdb can be used inside the env, not too sure how that works
            havent tried to run gdb, but could be cool, maybe i find a client
            use gdb server and push it over an internal address, instead of running
            screen multiplexer inside of userland
*/
