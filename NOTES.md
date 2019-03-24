# KFS
## Goals
### KFS1 : Boot
- Install GRUB on a virtual image.
- Write an ASM boot code that handles multiboot header, and use GRUB to init and call the main function of the kernel itself.
- Write basic kernel code in the chosen language.
- Compile it with the correct flags, and link it to make it bootable.
- Once all of the above steps are done, you can write some helpers like kernel types or basic functions (strlen, strcmp, ...).
- Finally, code the interface between your kernel and the screen.
### KFS2 : GDT & Stack
- You must create a _Global Descriptor Table_
- Your GDT must contain:
	- Kernel Code
	- Kernel Data
	- Kernel stack
	- User code
	- User data
	- User stack
- You must declare your GDT to the BIOS
- The GDT must be set at address 0x00000800
### KFS3 : Memory
- Enable memory paging in your Kernel
- Code a memory structure that handles paging and memory rights (Careful, you don’t have the tools yet to know who’s accessing to your memory, so all of this is pure theory at the moment)
- Define Kernel and User space
- Implement a function to create/get memory pages
- Implement functions to allocate, free and get the size of a variable.
- Implement those functions for virtual and physical memory
- Handle "kernel panics" (Print then stops the kernel)
### KFS4 : Interrupts
- Create an Interrupts Descriptor Table, fill it and register it
- Implement a signal-callback system on your Kernel API
- Implement an interface to schedule signals
- Implement an interface to clean registers before a panic / halt
- Implement an interface to save the stack before a panic
When you’re done with all of that, You’ll have to implement a keyboard handler, using the IDT.
### KFS5 : Processes
- A full structure containing data about processes. That includes:
	- A PID.
	- Status (Run, zombie, thread)
	- Pointers to father and children
	- Stack and heap of a process. (More information below)
	- Currents signals (Queue list)
	- Owner id (User)
- With that structure filled and dusted, you will need to implement the following functions:
	- A function to queue a signal to a processus, delivered on the next CPU tick
	- Sockets communication helpers between processes
	- Functions to work on the memory of a process.
	- A Function to copy an entire process (fork)
- On top of that, you need to code the followings helpers, in order to prepare the syscalls:
	- `wait`
	- `exit`
	- `getuid`
	- `signal`
	- `kill`

All of the functions above are meant to work like on any UNIX system.

Your kernel must handle multitasking. There are many ways to do it, you must choose a solution to implement that. Remember that your kernel is really small at the moment, and doesn’t handle that much processes.

- Functions like mmap, in order for a process to get his virtual memory.
- Link the IDT and the processes, in order to follow the future syscall signals.
- Create the BSS and data sectors in the process structure.

