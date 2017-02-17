import org.apache.hadoop.conf.Configuration;
import org.apache.hadoop.fs.Path;
import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Job;
import org.apache.hadoop.mapreduce.lib.input.FileInputFormat;
import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;
import org.apache.hadoop.util.GenericOptionsParser;

public class PurchaseNetwork {
    private static final String INTERMEDIATE_PATH = "intermediate_output";

    public static void main(String[] args) throws Exception {
        Configuration conf = new Configuration();
        String[] otherArgs = new GenericOptionsParser(conf, args).getRemainingArgs();

        if (otherArgs.length != 2) {
            System.err.println("Usage: PurchaseNetwork <in> <out>");
            System.exit(2);
        }

        Job job1 = new Job(conf, "PurchaseNetwork");
        job1.setJarByClass(PurchaseNetwork.class);
        job1.setMapperClass(PurchaseNetworkMapper.class);
        job1.setReducerClass(PurchaseNetworkReducer.class);
        job1.setOutputKeyClass(PurchaseNetworkKey.class);
        job1.setOutputValueClass(IntWritable.class);
        FileInputFormat.addInputPath(job1, new Path(otherArgs[0]));
        FileOutputFormat.setOutputPath(job1, new Path(INTERMEDIATE_PATH));
        job1.waitForCompletion(true);

        Job job2 = new Job(conf, "PurchaseNetworkCounter");
        job2.setJarByClass(PurchaseNetwork.class);
        job2.setMapperClass(PurchaseNetworkCounterMapper.class);
        job2.setReducerClass(PurchaseNetworkCounterReducer.class);
        job2.setOutputKeyClass(NullWritable.class);
        job2.setOutputValueClass(Text.class);
        FileInputFormat.addInputPath(job2, new Path(INTERMEDIATE_PATH));
        FileOutputFormat.setOutputPath(job2, new Path(otherArgs[1]));
        job2.waitForCompletion(true);
    }
}
