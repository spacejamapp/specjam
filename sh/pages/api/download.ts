import fs from "fs";
import path from "path";
import { NextApiRequest, NextApiResponse } from "next";

export default function handler(req: NextApiRequest, res: NextApiResponse) {
    const filePath = path.join(process.cwd(), 'public', 'download.sh');
    if (!fs.existsSync(filePath)) {
        res.status(404).json({ error: 'File not found' });
        return;
    }
    fs.readFile(filePath, (err, data) => {
        if (err) {
            res.status(500).json({ error: 'Error reading file' });
            return;
        }
        res.setHeader('Content-Type', 'text/plain');
        res.setHeader('Content-Disposition', 'attachment; filename=download.sh');
        res.send(data);
    });
}