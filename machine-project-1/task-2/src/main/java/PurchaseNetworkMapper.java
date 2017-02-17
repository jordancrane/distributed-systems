import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;
import org.paukov.combinatorics3.Generator;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class PurchaseNetworkMapper extends Mapper<Object, Text, PurchaseNetworkKey, IntWritable> {
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
