import { useEffect, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import Button from "../components/Button";

const OutputPage = () => {
    const location = useLocation();
    const testData = location.state?.testData;
    const navigate = useNavigate();

    const [objectUrl, setObjectUrl] = useState<string | null>(null);
    const objectUrlRef = useRef<string | null>(null); 

    useEffect(() => {
        if (!testData) return;

        const byteCharacters = atob(testData);
        const byteNumbers = Array.from(byteCharacters).map(c => c.charCodeAt(0));
        const byteArray = new Uint8Array(byteNumbers);
        const blob = new Blob([byteArray], { type: "application/pdf" });

        const url = URL.createObjectURL(blob);
        setObjectUrl(url);
        objectUrlRef.current = url;

        // cleanup on unmount
        return () => {
            if (objectUrlRef.current) {
                URL.revokeObjectURL(objectUrlRef.current);
            }
        };
    }, [testData]);

    const handleOpen = () => {
        if (objectUrl) {
            window.open(objectUrl, '_blank');
        }
    };

    const handleDownload = () => {
        if (objectUrl) {
            const link = document.createElement("a");
            link.href = objectUrl;
            link.download = gen_pdf_name();
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        }
    };

    const gen_pdf_name = (): string => {
        const d = new Date();
        const yyyy = d.getFullYear().toString().padStart(4, '0');
        const mm = (d.getMonth() + 1).toString().padStart(2, '0');
        const dd = d.getDate().toString().padStart(2, '0');
        return `english_test-${yyyy}_${mm}_${dd}.pdf`;
    };

    return (
        <div>
            <div className="m-[10vh] text-center font-bold">
                <h2>テストの作成が完了しました。</h2>
            </div>
            

            <div className="fixed bottom-[10vh] left-0 right-0 text-center flex-col-reverse">
                <p>
                    <Button className="w-[60vw] py-[1vh] mt-[2.5vh] rounded-[2vw] text-center text-white bg-blue-400" onClick={handleOpen}>
                        ブラウザで開く
                    </Button>
                </p>
                <p>
                    <Button className="w-[60vw] py-[1vh] mt-[2.5vh] rounded-[2vw] text-center text-white bg-blue-400" onClick={handleDownload}>
                        PDFをダウンロード
                    </Button>
                </p>
                <p>
                    <Button 
                        className="w-[60vw] py-[1vh] mt-[2.5vh] rounded-[2vw] text-center border text-black bg-white-400" 
                        onClick={() => navigate("/")}
                    > 
                        戻る
                    </Button>
                </p>
            </div>
        </div>
    );
};

export default OutputPage;
