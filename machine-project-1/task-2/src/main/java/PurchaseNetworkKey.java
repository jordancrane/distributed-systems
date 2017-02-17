import org.apache.hadoop.io.Text;
import org.apache.hadoop.io.WritableComparable;

import java.io.DataInput;
import java.io.DataOutput;
import java.io.IOException;

public class PurchaseNetworkKey implements WritableComparable<PurchaseNetworkKey> {
    private Text first;
    private Text second;

    PurchaseNetworkKey() {
        this.first = new Text();
        this.second = new Text();
    }

    PurchaseNetworkKey(String first, String second) {
        if (first.compareTo(second) <= 0) {
            this.first = new Text(first);
            this.second = new Text(second);
        } else {
            this.first = new Text(second);
            this.second = new Text(first);
        }
    }

    @Override
    public int compareTo(PurchaseNetworkKey o) {
        if (first.compareTo(o.first) == 0) {
            return second.compareTo(o.second);
        } else {
            return first.compareTo(o.first);
        }
    }

    @Override
    public void write(DataOutput dataOutput) throws IOException {
        first.write(dataOutput);
        second.write(dataOutput);
    }

    @Override
    public void readFields(DataInput dataInput) throws IOException {
        first.readFields(dataInput);
        second.readFields(dataInput);
    }

    public String toString() {
        return "(" + first.toString() + ", " + second.toString() + ")";
    }
}