import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;
import org.apache.hadoop.mapreduce.Reducer;
import org.junit.Test;
import org.mockito.Mockito;

import java.io.IOException;
import java.util.Arrays;

import static org.junit.Assert.assertEquals;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;

public class PurchaseNetworkKeyTest {

    @Test
    public void testAlphabetical() {
        PurchaseNetworkKey key = null;

        key = new PurchaseNetworkKey("Apple", "Banana");
        assertEquals("(Apple, Banana)", key.toString());

        key = new PurchaseNetworkKey("Banana", "Apple");
        assertEquals("(Apple, Banana)", key.toString());
    }
}