import { useState } from "react";
import { useNavigate } from "react-router-dom";

import Header from "../components/Header";

import axios from "axios";
import "../style/MainPage.css";
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
                "http://54.178.56.216:3000/generate-test", 
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
        <div className="whole_content">
            <Header/>

            <div className="main_content">
                <div className="english_word_book">
                    英単語帳を選択してください
                    
                    <br />
                    <select 
                        value={englishWordBook}
                        onChange={(e) => setEnglishWordBook(e.target.value)}
                    >
                        <option value="">英単語帳を選択してください</option>
                        <option value="shisutan">システム英単語［シス単］（5訂版）</option>
                    </select>
                </div>
                
                <div className="test_range">
                    <br className="test_range_explanation"/>
                        範囲を指定してください
                    <br />

                    <div>
                        開始番号：
                        <input 
                            type="tel" 
                            value={startNumber}
                            onChange={(e) => setStartNumber(Number(e.target.value))}
                        /> 
                        <br />
                        終了番号：
                        <input 
                            type="tel" 
                            value={endNumber}
                            onChange={(e) => setEndNumber(Number(e.target.value))}
                        />
                    </div>
                </div>

                <br />

                <div className="Number_of_questions">
                    <div className="Number_of_questions_explanation">
                        問題数を指定してください

                        <br />

                        <div>
                            問題数：
                            <input 
                                type="tel" 
                                value={times}
                                onChange={(e) => setTimes(Number(e.target.value))}
                            />
                        </div>
                    </div>
                </div>

                <br />

                
                <Button onClick={handleSubmit} disabled={loading}>
                    {loading ? "生成中...しばらくお待ちください" : "上記の内容でテストを作成する"}  
                </Button>

            </div>

            {error && <div style={{ color: "red" }}>{error}</div>}

            <br />

        </div>
    );
};

export default MainPage
