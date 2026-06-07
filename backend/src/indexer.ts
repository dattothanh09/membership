import { SorobanRpc } from "@stellar/stellar-sdk";
import { db } from "./db";

// Thay bằng URL RPC thực tế (Testnet/Mainnet)
const server = new SorobanRpc.Server("https://soroban-testnet.stellar.org");
const CONTRACT_ID = "C...YOUR_CONTRACT_ADDRESS_HERE...";

export async function startIndexer() {
    console.log("Đang khởi động Soroban Event Indexer...");
    let lastLedger = await server.getLatestLedger();

    setInterval(async () => {
        try {
            const eventsResponse = await server.getEvents({
                startLedger: lastLedger.sequence,
                filters: [{ contractIds: [CONTRACT_ID] }]
            });

            for (const event of eventsResponse.events) {
                const txHash = event.txHash;
                const eventType = event.topic[0].value().toString(); // 'mint', 'transfer', 'checkin'

                // Bỏ qua nếu giao dịch đã được lưu
                const existingTx = await db.transaction.findUnique({ where: { txHash } });
                if (existingTx) continue;

                if (eventType === 'checkin') {
                    const userWallet = event.topic[1].value().toString();
                    await db.transaction.create({
                        data: {
                            txHash: txHash,
                            txType: 'CHECKIN',
                            fromWallet: userWallet,
                            amount: 1,
                        }
                    });
                    console.log(`[Indexer] Check-in lưu thành công: ${userWallet}`);
                } 
                // Cần parse thêm logic cho 'transfer' và 'mint' ở đây...
            }
            lastLedger = await server.getLatestLedger();
        } catch (error) {
            console.error("[Indexer] Lỗi quét ledger:", error);
        }
    }, 5000); // Quét lại sau mỗi 5 giây
}