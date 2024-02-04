const http = require('http');

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Hello from Server 3\n');
});

const PORT = 5003;
server.listen(PORT, () => {
    console.log(`Server 3 running on port ${PORT}`);
});
