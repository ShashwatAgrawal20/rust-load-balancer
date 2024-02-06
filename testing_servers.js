const http = require('http');

// Function to create a server
function createServer(port, number) {
    return http.createServer((req, res) => {
        res.writeHead(200, { 'Content-Type': 'text/plain' });
        res.end(`Hello from Server ${number}\n`);
    });
}

// Create three servers
const server1 = createServer(5001, 1);
const server2 = createServer(5002, 2);
const server3 = createServer(5003, 3);

// Listen on ports
server1.listen(5001, () => {
    console.log('Server 1 running on port 5001');
});

server2.listen(5002, () => {
    console.log('Server 2 running on port 5002');
});

server3.listen(5003, () => {
    console.log('Server 3 running on port 5003');
});

