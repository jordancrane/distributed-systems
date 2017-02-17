import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Reducer;
import org.junit.Test;
import org.mockito.Mockito;

import java.io.IOException;
import java.util.Arrays;

import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;

public class PurchaseNetworkReducerTest {
    @Test
    public void testReducer() throws IOException, InterruptedException {
        PurchaseNetworkReducer reducer = new PurchaseNetworkReducer();
        Reducer.Context context = mock(Reducer.Context.class);

        reducer.reduce(
                new PurchaseNetworkKey("Whitey Toothpaste", "Best Bread"),
                Arrays.asList(new IntWritable(1), new IntWritable(1)),
                context
        );

        verify(context, Mockito.times(1)).write(eq(new Text("(Best Bread, Whitey Toothpaste)")), eq(new IntWritable(2)));
    }
}