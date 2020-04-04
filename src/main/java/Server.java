import java.io.IOException;
import java.net.InetSocketAddress;
import java.net.ServerSocket;
import java.net.Socket;

public class Server {

    public void listen() throws IOException {
        ServerSocket serverSocket;

        serverSocket = new ServerSocket();
        int port = 7743;
        InetSocketAddress address = new InetSocketAddress(port);
        System.out.println("Listening on port: " + address);
        serverSocket.bind(address);
        //while(true) {
        for(int i=0; i<1; i++) {
            System.out.println("Accept");
            Socket socket = serverSocket.accept();
            System.out.println("Zglosil sie klient: " + socket.getInetAddress());
            System.out.println("Port: " + socket.getPort() + ", " + socket.getLocalPort());

            Client client = new Client(socket.getPort());
            client.greet();
        }
    }

    public static void main(String[] args) {
        try {
            var server = new Server();
            server.listen();
        }
        catch (Exception ex)
        {
            System.out.println(ex);
            ex.printStackTrace();
        }
    }
}
