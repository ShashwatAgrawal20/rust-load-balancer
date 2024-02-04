const http = require('http');

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Hello from Server 2\n');
});

const PORT = 5002;
server.listen(PORT, () => {
    console.log(`Server 2 running on port ${PORT}`);
});
