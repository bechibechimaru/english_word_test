import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import MainPage from './pages/MainPage';
import OutputPage from './pages/OutputPage';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<MainPage />} />
        <Route path="/output" element={<OutputPage />} />
      </Routes>
    </Router>
  );
}

export default App;
