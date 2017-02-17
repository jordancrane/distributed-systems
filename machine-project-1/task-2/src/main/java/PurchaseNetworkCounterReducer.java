import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Reducer;

import java.io.IOException;

public class PurchaseNetworkCounterReducer extends Reducer<NullWritable, Text, NullWritable, Text> {
    private Integer count = 0;

    protected void reduce(NullWritable _, Iterable<Text> values, Context context)
            throws IOException, InterruptedException {
        for (Text value : values) {
            count += 1;
            context.write(NullWritable.get(), value);
        }

        context.write(NullWritable.get(), new Text("Total Pairs: " + count));
    }
}