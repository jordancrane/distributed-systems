JFLAGS = -g -d
JC = javac
NAME = WordCount

default: jar

jar: classes 
	jar -cvf $(NAME).jar -C $(NAME)/ .

classes:
	$(JC) -classpath $(HADOOP_CLASSPATH) $(JFLAGS) $(shell pwd)/$(NAME) *.java

clean:
	$(RM) *.class
