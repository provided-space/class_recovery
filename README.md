> Retrieve Java Class files from a memory dump

# Class Recovery

This tool helps you find class files within a dumped java process. <br>
Encrypted classes are supported to an extent where the decrypted buffer has reached the JVM.

## How does it work?

The `class_recovery` searches for all matches with the byte sequence `0xCAFEBABE` in the file buffer and from there on tries to parse the classes. (See [The class File Format](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html))<br>
This rather primitive approach allows us to gather only the relevant information and calculating the end of the class file, without the risk of failing due to class file manipulation, such as certain crashers / obfuscation techniques.

Currently, all class file formats are supported, up to Java version 22. This project uses https://github.com/openjdk/jdk ClassFileParser as reference for the implementation.

## Impression

![Impression](assets/impression.gif?raw=true)
