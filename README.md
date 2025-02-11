# Stack Machine
A basic stack machine math interpreter created without following *Crafting Interpreters*, instead using some concepts I have learnt from *Compilers: Principles, Techniques, and Tools* (the Dragon Book) (which does not teach interpreting)

Here I "compile" math expressions to this basic stack machine language. 
Each instruction gets interpreted by the interpreter until there are none left, leaving the answer of the expression on the stack.

<img width="388" alt="Screenshot 2025-02-11 at 8 39 36 pm" src="https://github.com/user-attachments/assets/f059bc6f-1741-45ed-911b-ec1288b88f0a" />

The graphical display of what the program is doing is created using [egui](https://www.egui.rs/)
