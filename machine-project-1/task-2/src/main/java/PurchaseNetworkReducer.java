import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Reducer;

import java.io.IOException;

public class PurchaseNetworkReducer extends Reducer<PurchaseNetworkKey, IntWritable, Text, IntWritable> {
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