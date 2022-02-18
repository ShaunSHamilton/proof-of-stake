// Handles parsing of quiz assets
const fs = require("fs/promises");
const { parseQuiz } = require("./utils/parser");

const quizObj = parseQuiz();

writeQuizToFile(quizObj);

async function writeQuizToFile(quizObj) {
  const FILE_PATH = "./assets/quiz.json";
  try {
    await fs.writeFile(FILE_PATH, JSON.stringify(quizObj), {
      flag: "w+",
    });
  } catch (e) {
    console.log("Error writing quiz to file");
    console.error(e);
  }
}
