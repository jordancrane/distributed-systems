# Machine Project 1 WordCount
Task 1 of machine project 1 for ECE499: Distributed Systems

## Building
1. Create and environment variable for your hadoop classpath called `$HADOOP_CLASSPATH`. To find your hadoop classpath, you can use the following command:

    ```console
    $ hadoop classpath
    ```
2. Run the make file

    ```console
    $ make
    ```

## Running
Place input files in the `input` folder, and run the program using the following commands:

```console
$ make clean
$ hadoop jar WordCount.jar input/ output/
```

Output will be in the `output` folder.

