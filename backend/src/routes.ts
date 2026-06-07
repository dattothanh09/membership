import { Router } from 'express';
import { db } from './db';

export const routes = Router();

// Lấy hồ sơ và lịch sử giao dịch của một hội viên
routes.get('/users/:walletAddress', async (req, res) => {
    const { walletAddress } = req.params;
    
    try {
        const profile = await db.user.findUnique({
            where: { walletAddress }
        });

        const history = await db.transaction.findMany({
            where: {
                OR: [
                    { fromWallet: walletAddress },
                    { toWallet: walletAddress }
                ]
            },
            orderBy: { timestamp: 'desc' }
        });

        res.json({ success: true, data: { profile, history } });
    } catch (error) {
        res.status(500).json({ success: false, error: 'Lỗi truy xuất dữ liệu' });
    }
});

// Thêm mới hội viên (Đăng ký tài khoản Web2 trước khi mua gói)
routes.post('/users', async (req, res) => {
    const { walletAddress, fullName, phoneNumber } = req.body;
    try {
        const user = await db.user.create({
            data: { walletAddress, fullName, phoneNumber }
        });
        res.json({ success: true, data: user });
    } catch (error) {
        res.status(400).json({ success: false, error: 'Ví đã tồn tại hoặc dữ liệu sai' });
    }
});