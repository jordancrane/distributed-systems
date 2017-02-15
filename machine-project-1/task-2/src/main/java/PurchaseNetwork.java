import org.apache.hadoop.conf.Configuration;
import org.apache.hadoop.fs.Path;
import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.io.WritableComparable;
import org.apache.hadoop.mapreduce.Job;
import org.apache.hadoop.mapreduce.Mapper;
import org.apache.hadoop.mapreduce.Reducer;
import org.apache.hadoop.mapreduce.lib.input.FileInputFormat;
import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;
import org.apache.hadoop.util.GenericOptionsParser;

import org.paukov.combinatorics3.Generator;

import java.io.DataInput;
import java.io.DataOutput;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class PurchaseNetwork {

    static class PurchaseNetworkKey implements WritableComparable<PurchaseNetworkKey> {
        Text left;
        Text right;

        PurchaseNetworkKey() {
            this.left = new Text();
            this.right = new Text();
        }

        PurchaseNetworkKey(String left, String right) {
            this.left = new Text(left);
            this.right = new Text(right);
        }

        @Override
        public int compareTo(PurchaseNetworkKey o) {
            if (left.compareTo(o.left) == 0) {
                return right.compareTo(o.right);
            } else {
                return left.compareTo(o.left);
            }
        }

        @Override
        public void write(DataOutput dataOutput) throws IOException {
            left.write(dataOutput);
            right.write(dataOutput);
        }

        @Override
        public void readFields(DataInput dataInput) throws IOException {
            left.readFields(dataInput);
            right.readFields(dataInput);
        }

        public String toString() {
            return "(" + left.toString() + ", " + right.toString() + ")";
        }
    }

    static class PurchaseNetworkMapper extends Mapper<Object, Text, PurchaseNetworkKey, IntWritable> {
        protected void map(Object _, Text value, Context context)
                throws IOException, InterruptedException {

            List<String> items = Arrays.asList(value.toString().split(", "));

            List<PurchaseNetworkKey> keys = Generator.combination(items)
                    .simple(2)
                    .stream()
                    .map((pair) -> new PurchaseNetworkKey(pair.get(0), pair.get(1)))
                    .collect(Collectors.toList());

            for (PurchaseNetworkKey key : keys) {
                context.write(key, new IntWritable(1));
            }
        }
    }

    static class PurchaseNetworkReducer extends Reducer<PurchaseNetworkKey, IntWritable, Text, IntWritable> {
        IntWritable result = new IntWritable();

        protected void reduce(PurchaseNetworkKey key, Iterable<IntWritable> values, Context context)
                throws IOException, InterruptedException {
            int sum = 0;
            for (IntWritable val : values) {
                sum += val.get();
            }
            result.set(sum);
            context.write(new Text(key.toString()), result);
        }
    }

    public static void main(String[] args) throws Exception {
        Configuration conf = new Configuration();
        String[] otherArgs = new GenericOptionsParser(conf, args).getRemainingArgs();

        if (otherArgs.length != 2) {
            System.err.println("Usage: PurchaseNetwork <in> <out>");
            System.exit(2);
        }

        Job job = new Job(conf, "PurchaseNetwork");
        job.setJarByClass(PurchaseNetwork.class);
        job.setMapperClass(PurchaseNetworkMapper.class);
        job.setReducerClass(PurchaseNetworkReducer.class);

        job.setOutputKeyClass(PurchaseNetworkKey.class);
        job.setOutputValueClass(IntWritable.class);

        FileInputFormat.addInputPath(job, new Path(otherArgs[0]));
        FileOutputFormat.setOutputPath(job, new Path(otherArgs[1]));

        System.exit(job.waitForCompletion(true) ? 0 : 1);
    }
}
