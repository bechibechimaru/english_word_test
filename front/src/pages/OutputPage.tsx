import { useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import '../style/OutputPage.css';

const OutputPage = () => {
    const location = useLocation();
    const testData = location.state?.testData;
    const navigate = useNavigate();

    useEffect(() => {
        if (!testData) return;

        // base64 → Blob
        const byteCharacters = atob(testData);
        const byteNumbers = Array.from(byteCharacters).map(char => char.charCodeAt(0));
        const byteArray = new Uint8Array(byteNumbers);
        const blob = new Blob([byteArray], { type: "application/pdf" });

        // Blob → Object URL → ダウンロードリンク
        const link = document.createElement("a");
        link.href = URL.createObjectURL(blob);
        const pdf_name: string = gen_pdf_name();
        link.download = pdf_name;
        link.click();

        // メモリ解放
        URL.revokeObjectURL(link.href);
    }, [testData]);


    const gen_pdf_name = (): string => {
        const current_time = new Date();

        const year = current_time.getFullYear().toString().padStart(4, '0');
        const month = (current_time.getMonth() + 1).toString().padStart(2, '0');
        const day = current_time.getDate().toString().padStart(2, '0');
        const dateText = `${year}_${month}_${day}`;

        const pdf_name = (`english_test-${dateText}`);
        return pdf_name
    }

    return (
        <div className="outputpage_main">
            <h2>英単語テストのダウンロードが完了いたしました。</h2>
            
            <button className="return-button" onClick={() => navigate("/")}>戻る</button>
            
            
        </div>
    );
};

export default OutputPage;
