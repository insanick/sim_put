# sim_put

A library created out of frustration from constantly using the rust std::io calls. 

Already published, may be updated with time, may not. I develop for fun :D


Quick Start: 
```
use sim_put::Input;

fn main() {
    let no_prompt_input: String = Input::input();

    let prompt_input: String = Input::prompted_input("Here is your prompt");
}
```
