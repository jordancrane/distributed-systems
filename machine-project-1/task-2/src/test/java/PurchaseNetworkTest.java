import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;
import org.apache.hadoop.mapreduce.Reducer;
import org.junit.Test;
import org.mockito.Mockito;
import org.paukov.combinatorics3.Generator;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;

public class PurchaseNetworkTest {
//    @Test
//    public void evaluatesExpression() {
//        List<String> items = Generator.combination("Apple", "Milk", "Avocado")
//                .simple(2)
//                .stream()
//                .map((pair) -> "(" + pair.get(0) + ", " + pair.get(1) + ")")
//                .collect(Collectors.toList());
//
//        for (String item : items){
//            System.out.println(item);
//        }
//    }

    @Test
    public void testMapper() throws InterruptedException, IOException {
        PurchaseNetwork.PurchaseNetworkMapper mapper = new PurchaseNetwork.PurchaseNetworkMapper();
        Mapper.Context context = mock(Mapper.Context.class);

        mapper.map(NullWritable.get(), new Text("Whitey Toothpaste, Best Bread, Fluffy Pizza"), context);
        verify(context, Mockito.times(3)).write(any(PurchaseNetwork.PurchaseNetworkKey.class), eq(new IntWritable(1)));

//        verify(context).write(new PurchaseNetwork.PurchaseNetworkKey("Whitey Toothpaste", "Best Bread"), new IntWritable(1));
//        verify(context).write(new PurchaseNetwork.PurchaseNetworkKey("Whitey Toothpaste", "Fluffy Pizza"), new IntWritable(1));
//        verify(context).write(new PurchaseNetwork.PurchaseNetworkKey("Best Bread", "Fluffy Pizza"), new IntWritable(1));
    }

    @Test
    public void testReducer() throws IOException, InterruptedException {
        PurchaseNetwork.PurchaseNetworkReducer reducer = new PurchaseNetwork.PurchaseNetworkReducer();
        Reducer.Context context = mock(Reducer.Context.class);

        reducer.reduce(
                new PurchaseNetwork.PurchaseNetworkKey("Whitey Toothpaste", "Best Bread"),
                Arrays.asList(new IntWritable(1), new IntWritable(1)),
                context
        );

        verify(context, Mockito.times(1)).write(eq(new Text("(Best Bread, Whitey Toothpaste)")), eq(new IntWritable(2)));
    }
}