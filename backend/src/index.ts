import express from 'express';
import { routes } from './routes';
import { startIndexer } from './indexer';

const app = express();
app.use(express.json());

// Gắn các API routes
app.use('/api', routes);

const PORT = process.env.PORT || 3000;

app.listen(PORT, () => {
    console.log(`Backend Server đang chạy tại http://localhost:${PORT}`);
    
    // Khởi chạy trình lắng nghe Blockchain ngay khi server start
    startIndexer();
});