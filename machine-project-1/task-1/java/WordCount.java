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
import org.apache.hadoop.mapreduce.lib.input.KeyValueTextInputFormat;
import org.apache.hadoop.mapreduce.lib.input.FileSplit;
import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;
import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.LongWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.util.StringUtils;
import org.apache.hadoop.util.GenericOptionsParser;


public class WordCount
{
    private static final String INTERMEDIATE_PATH = "intermediate_output";

	public static void main(String[] args) throws Exception
	{
        // Job configurations
		Configuration conf = new Configuration();

        // Argument parser
		String[] otherArgs = new GenericOptionsParser(conf, args).getRemainingArgs();
		if (otherArgs.length != 2)
		{
			System.err.println("Usage: WordCount <in> <out>");
			System.exit(2);
		}

        // MapReduce 1st pass
		Job job1 = new Job(conf, "WordCount");
		job1.setJarByClass(WordCount.class);

		job1.setMapperClass(WordCountMapper.class);
		job1.setReducerClass(WordCountReducer.class);

		job1.setOutputKeyClass(Text.class);
		job1.setOutputValueClass(IntWritable.class);

		FileInputFormat.addInputPath(job1, new Path(otherArgs[0]));
		FileOutputFormat.setOutputPath(job1, new Path(INTERMEDIATE_PATH));

		job1.waitForCompletion(true);

        // MapReduce 2nd pass
		Job job2 = new Job(conf, "WordCountSum");
		job2.setJarByClass(WordCount.class);
        job2.setInputFormatClass(KeyValueTextInputFormat.class);

		job2.setMapperClass(WordCountSumMapper.class);
		job2.setReducerClass(WordCountReducer.class);

		job2.setOutputKeyClass(Text.class);
		job2.setOutputValueClass(IntWritable.class);

		FileInputFormat.addInputPath(job2, new Path(INTERMEDIATE_PATH));
		FileOutputFormat.setOutputPath(job2, new Path(otherArgs[1]));

		System.exit(job2.waitForCompletion(true) ? 0 : 1);
	}

	public static class WordCountMapper extends Mapper<LongWritable, Text, Text, IntWritable>
	{
		private Text word = new Text();

		public void map(LongWritable _, Text lineText, Context context)
			throws IOException, InterruptedException
		{

			String line = lineText.toString().toLowerCase();
			Text currentWord = new Text();

			for (String word : line.split("\\W+"))
			{
				if (word.isEmpty())
				{
					continue;
				}
				currentWord = new Text(word);
				context.write(currentWord, new IntWritable(1));
			}
		}
	}

	public static class WordCountSumMapper extends Mapper<Text, Text, Text, IntWritable>
	{
		private Text total  = new Text("Total Words:");
		private Text unique = new Text("Unique Words:");

        private int total_count;
        private int value_int;

		public void map(Text key, Text value, Context context)
			throws IOException, InterruptedException
		{
            value_int = Integer.parseInt(value.toString());
            total_count = value_int;

            context.write(key, new IntWritable(value_int));
            context.write(total, new IntWritable(total_count));
            context.write(unique, new IntWritable(1));
		}
	}

	public static class WordCountReducer extends Reducer<Text, IntWritable, Text, IntWritable>
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
