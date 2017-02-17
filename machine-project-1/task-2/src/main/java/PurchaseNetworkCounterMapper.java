import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;

import java.io.IOException;

public class PurchaseNetworkCounterMapper extends Mapper<Object, Text, NullWritable, Text> {
    protected void map(Object _, Text value, Context context)
            throws IOException, InterruptedException {
        context.write(NullWritable.get(), value);
    }
}