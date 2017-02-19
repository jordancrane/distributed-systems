JFLAGS = -g -d
JC = javac
NAME = WordCount
JAVA_PATH = java

default: jar

jar: classes 
	jar -cvf $(NAME).jar -C build/ .

classes:
	sudo $(JC) -cp $(HADOOP_CLASSPATH) $(JAVA_PATH)/$(NAME).java -d build -Xlint:-path

clean:
	$(RM) -r *.class *output build/*
