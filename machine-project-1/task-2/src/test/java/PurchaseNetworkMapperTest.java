import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.NullWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;
import org.junit.Test;
import org.mockito.Mockito;

import java.io.IOException;

import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;

public class PurchaseNetworkMapperTest {
    @Test
    public void testMapper() throws InterruptedException, IOException {
        PurchaseNetworkMapper mapper = new PurchaseNetworkMapper();
        Mapper.Context context = mock(Mapper.Context.class);

        mapper.map(NullWritable.get(), new Text("Whitey Toothpaste, Best Bread, Fluffy Pizza"), context);
        verify(context, Mockito.times(3)).write(any(PurchaseNetworkKey.class), eq(new IntWritable(1)));
    }
}