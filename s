nasm -felf64 -o main.a dub.asm
gcc main.a -o main -no-pie
./main