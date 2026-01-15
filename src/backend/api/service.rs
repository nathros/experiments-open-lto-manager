use dioxus::fullstack::TextStream;
use dioxus::prelude::*;

use std::process::Command;
use std::process::Stdio;

/*#[get("/api/test_stream")]
pub async fn text_stream() -> Result<TextStream> {
    let mut count = 0;

    // We can create a new text stream with `spawn`
    Ok(TextStream::spawn(move |tx| async move {
        // Send a message with `unbounded_send`
        while tx
            .unbounded_send(format!("Hello, world! {}", count))
            .is_ok()
        {
            count += 1;

            // and then wait a bit
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }))
}*/

#[get("/api/test_stream")]
pub async fn text_stream() -> Result<TextStream> {
    // We can create a new text stream with `spawn`
    Ok(TextStream::spawn(move |tx| async move {
        use std::io::{BufRead, BufReader};

        let mut cmd = Command::new("./test.sh")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdout = cmd.stdout.take().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            let stderr = cmd.stderr.take().unwrap();
            let stderr_reader = BufReader::new(stderr);
            let stderr_lines = stderr_reader.lines();

            for line in stdout_lines {
                if tx.is_closed() {
                    cmd.kill();
                    break;
                }
                tx.unbounded_send(format!("{}", line.unwrap())).is_ok();
            }

            for line in stderr_lines {
                if tx.is_closed() {
                    cmd.kill();
                    break;
                }

                tx.unbounded_send(format!("{}", line.unwrap())).is_ok();
            }
        }

        cmd.wait().unwrap(); // exit code here
    }))
}

#[get("/api/test_stream2")]
pub async fn text_stream2() -> Result<TextStream> {
    // We can create a new text stream with `spawn`
    Ok(TextStream::spawn(move |tx| async move {
        use std::io::{BufRead, BufReader};

        // bash -c "cat a.txt | tail -f a.txt"

        let vvv = &["-c", "cat a.txt && tail -f a.txt"];
        let mut cmd = Command::new("bash")
            .args(vvv)
            //.arg("\"cat a.txt && tail -f a.txt\"")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdout = cmd.stdout.take().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            let stderr = cmd.stderr.take().unwrap();
            let stderr_reader = BufReader::new(stderr);
            let stderr_lines = stderr_reader.lines();

            for line in stdout_lines {
                if tx.is_closed() {
                    cmd.kill();
                    break;
                }
                tx.unbounded_send(format!("{}", line.unwrap())).is_ok();
            }

            for line in stderr_lines {
                if tx.is_closed() {
                    cmd.kill();
                    break;
                }

                tx.unbounded_send(format!("{}", line.unwrap())).is_ok();
            }
        }

        cmd.wait().unwrap(); // exit code here
    }))
}
