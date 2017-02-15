package org.myorg;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.net.URI;
import java.util.HashSet;
import java.util.Set;
import java.util.regex.Pattern;
import java.util.ArrayList;
import org.apache.hadoop.conf.Configuration;
import org.apache.hadoop.conf.Configured;
import org.apache.hadoop.util.Tool;
import org.apache.hadoop.util.ToolRunner;
import org.apache.hadoop.mapreduce.Job;
import org.apache.hadoop.mapreduce.Mapper;
import org.apache.hadoop.mapreduce.Reducer;
import org.apache.hadoop.fs.Path;
import org.apache.hadoop.mapreduce.lib.input.FileInputFormat;
import org.apache.hadoop.mapreduce.lib.input.FileSplit;
import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;
import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.LongWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.util.StringUtils;
import org.apache.log4j.Logger;

public class WordCount extends Configured implements Tool
{
    private static final Logger LOG = Logger.getLogger(WordCount.class);
    public static void main(String[] args) throws Exception
    {
        int res = ToolRunner.run(new WordCount(), args);
        System.exit(res);
    }

    public int run(String[] args) throws Exception
    {
        Job job = Job.getInstance(getConf(), "wordcount");
        job.setJarByClass(this.getClass());
        // Use TextInputFormat, the default unless job.setInputFormatClass is used
        FileInputFormat.addInputPath(job, new Path(args[0]));
        FileOutputFormat.setOutputPath(job, new Path(args[1]));
        job.setMapperClass(Map.class);
        job.setCombinerClass(Reduce.class);
        job.setReducerClass(Reduce.class);
        job.setOutputKeyClass(Text.class);
        job.setOutputValueClass(IntWritable.class);
        return job.waitForCompletion(true) ? 0 : 1;
    }

    public static class Map extends Mapper<LongWritable, Text, Text, IntWritable>
    {
        private Text word = new Text();
        private Text total = new Text("Total Words:");
        private Text unique = new Text("Unique Words:");
        private ArrayList<Text> seen = new ArrayList<Text>();
        private boolean caseSensitive = false;

        protected void setup(Mapper.Context context)
            throws IOException,
                   InterruptedException
                   {
                       Configuration config = context.getConfiguration();
                       this.caseSensitive = config.getBoolean("wordcount.case.sensitive", false);
                   }

        public void map(LongWritable offset, Text lineText, Context context)
            throws IOException, InterruptedException {
                String line = lineText.toString();
                if (!caseSensitive)
                {
                    line = line.toLowerCase();
                }
                Text currentWord = new Text();
                int unique_sum = 0;
                for (String word : line.split("\\W+"))
                {
                    if (word.isEmpty())
                    {
                        continue;
                    }
                    currentWord = new Text(word);
                    // Check if word is unique
                    if (!seen.contains(currentWord))
                    {
                        seen.add(currentWord);
                        unique_sum++;
                    }
                    context.write(currentWord, new IntWritable(1));
                    context.write(total, new IntWritable(1));
                }
                context.write(unique, new IntWritable(unique_sum));
            }
    }

    public static class Reduce extends Reducer<Text, IntWritable, Text, IntWritable>
    {
        @Override
            public void reduce(Text word, Iterable<IntWritable> counts, Context context)
            throws IOException, InterruptedException
            {
                int sum = 0;
                for (IntWritable count : counts)
                {
                    sum += count.get();
                }
                context.write(word, new IntWritable(sum));
            }
    }
}
