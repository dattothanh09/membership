import { PrismaClient } from '@prisma/client';

// Khởi tạo một instance duy nhất để tránh cạn kiệt connection pool
export const db = new PrismaClient();