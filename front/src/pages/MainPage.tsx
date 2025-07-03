import { useState } from "react";
import { useNavigate } from "react-router-dom";

import Header from "../components/Header";

import axios from "axios";
import Button from "../components/Button";

// 出力する英単語などを入力させるページ
type GenerateTestRespose = {
    test_data: string;
};

const MainPage = () => {
    const navigate = useNavigate();

    // フォームデータの状態を保存
    const [englishWordBook, setEnglishWordBook] = useState<string>("");
    const [startNumber, setStartNumber] = useState<number>(1);
    const [endNumber, setEndNumber] = useState<number>(100);
    const [times, setTimes] = useState<number>(10);
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    // フォーム送信処理
    const handleSubmit = async () => {
        setLoading(true);
        setError(null);

        const isValida = validate();
        if (!isValida) {
            setLoading(false);
            return;
        }

        try{
            const response = await axios.post<GenerateTestRespose>(
                "http://localhost:3000/generate-test", 
            {
                english_word_book: englishWordBook,
                times: times,
                start_number: startNumber,
                end_number: endNumber
            });

            // レスポンスデータの処理
            console.log(response.data);
            
            navigate("/output", { state: {testData: response.data.test_data } });

        }catch (error) {
            setError("テスト作成に失敗しました。")
        } finally {
            setLoading(false);
        }
    };

    const validate = (): boolean => {
        if (englishWordBook === "") {
            setError("英単語帳を選択してください");
            return false;
        }
        else if (startNumber === 0 ) {
            setError("開始番号は1から指定してください。");
            return false;
        }
        else if (endNumber === 0 ) {
            setError("適切な終了番号を入力指定してください");
            return false;
        }
        else if (times === 0) {
            setError("問題数を設定してください");
            return false;
        }
        else if (startNumber >= endNumber) {
            setError("終了番号は開始番号より後の数値を入力してください");
            return false;
        }

        setError(null);
        return true;
    };

    return (
        <div className="grid place-content-center">
            <Header />

                <div className="mx-6 content-center">
                    <div className="m-6">
                        <select 
                            className="mt-1 py-3 pl-2 pr-20 border rounded-[2vw]"
                            value={englishWordBook}
                            onChange={(e) => setEnglishWordBook(e.target.value)}
                        >
                            <option value="">英単語帳を選択</option>
                            <option value="shisutan">システム英単語［シス単］（5訂版）</option>
                            <option value="target1900">英単語ターゲット1900（6訂版）</option>
                            <option value="stock4500">英単語Stock4500（428~506）</option>
                            <option value="target1200">英単語ターゲット1200 改訂版</option>
                        </select>
                    </div>

                    <div className="m-6 ">
                        <div className="pl-1">
                            開始番号
                        </div>
                        <input 
                            className="mt-1 py-3 pl-2 pr-30 border rounded-[2vw]"
                            type="tel" 
                            name="startNumber"
                            inputMode="numeric"
                            pattern="[0-9]*"
                            placeholder="開始番号を入力"
                            value={startNumber}
                            onChange={(e) => setStartNumber(Number(e.target.value.replace(/^0+/, '')))}
                        /> 
                    </div>
                    <div className="m-6">
                        <div className="pl-1">
                            終了番号
                        </div>

                        <input 
                            className="mt-1 py-3 pl-2 pr-30 border rounded-[2vw]"
                            type="tel" 
                            name="endNumber"
                            inputMode="numeric"
                            pattern="[0-9]*"
                            placeholder="終了番号を入力"
                            value={endNumber}
                            onChange={(e) => setEndNumber(Number(e.target.value.replace(/^0+/, '')))}
                        />
                    </div>

                    <div className="m-6">
                        <div className="pl-1">
                            問題数
                        </div>
                        <input 
                            className="mt-1 py-3 pl-2 pr-30 border rounded-[2vw]"
                            type="tel" 
                            inputMode="numeric"
                            pattern="[0-9]*"
                            value={times}
                            onChange={(e) => setTimes(Number(e.target.value.replace(/^0+/, '')))}
                        />
                        </div>
                </div>

                <Button 
                    className="text-white bg-blue-400 m-6 p-3 rounded-[2vw]"   
                    onClick={handleSubmit} 
                    disabled={loading} 
                    >
                    {loading ? "生成中...しばらくお待ちください" : "上記の内容でテストを作成する"}  
                </Button>

            {error && <div style={{ color: "red" }}>{error}</div>}

            <br />

        </div>
    );
};

export default MainPage
