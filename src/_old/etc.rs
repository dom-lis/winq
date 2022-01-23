
        let new_terminal_size = termion::terminal_size()?;
        if terminal_size != new_terminal_size {
            terminal_size = new_terminal_size;
            let (w, h) = terminal_size;
            child_stdin.send(format!("size:{},{}", w, h))?;
        }

        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            } else {
                return Err(Box::new(ChildError::Exit(status.code())));
            }
        }

        // collect terminal input (host_stdin)
        for hi in host_stdin.try_iter() {
            // relay terminal input to child (child_stdin)
            match hi {
                Ok(e) => if let Some(s) = input::repr_event(&e) {
                    child_stdin.send(s)?;
                },
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }

        // collect child stderr
        let child_errs = child_stderr.try_iter().collect::<Result<Vec<_>, std::io::Error>>()?;
        if !child_errs.is_empty() {
            // gracefully fail if there were lines in child stderr
            return Err(Box::new(ChildError::Stderr(child_errs)));
        }
        
        // collect child comms from child stdout
        for s in child_stdout.try_iter() {
            match s {
                Ok(line) => {
                    if line.chars().next() == Some('\t') {
                        match line.as_str() {
                            "\tflush" => {
                                mode = Mode::Text;
                                flushed_state = Some(state);
                                state = State::default();
                            },
                            "\ttext" => {
                                mode = Mode::Text;
                            },
                            "\tfg" => {
                                mode = Mode::Fg;
                            },
                            "\tbg" => {
                                mode = Mode::Bg;
                            }
                            "\tstyle" => {
                                mode = Mode::Style;
                            },
                            _ => {
                                return Err(Box::new(error::InternalError::BadMode(line)))
                            }
                        }
                    } else {
                        state.push(mode, line);
                    }
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }