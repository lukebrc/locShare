import lombok.extern.slf4j.Slf4j;

import java.io.IOException;
import java.io.OutputStream;
import java.net.Socket;

@Slf4j
public class Client {
    int port;

    public Client(int port) {
        this.port = port;
    }

    public void greet() throws IOException {
        //InetAddress inetAddress = InetAddress.getLocalHost();
        //InetSocketAddress inetAddress = new InetSocketAddress(port);
        Socket socket = new Socket("0.0.0.0", port);
        //Socket socket = new Socket(inetAddress);
        OutputStream outputStream = socket.getOutputStream();
        outputStream.write("Hello\n".getBytes());
        outputStream.close();
        socket.close();
    }
}
