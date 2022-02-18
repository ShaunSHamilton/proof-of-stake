const fs = require("fs");
const yaml = require("js-yaml");

const NUM_QUIZZES = 10;
const QUESTION_MARKER = "### Question";
const OPTIONS_MARKER = "### Options";
const RESULTS_MARKER = "### Results";
const NEXT_MARKER = `### `;

/**
 * Gets all content within a quiz
 * @param {string} fileContent - The quiz file content
 * @param {number} quizNumber - The number of the quiz
 * @returns {string} The content of the quiz
 */
function getQuizFromFileContent(fileContent, quizNumber) {
  const quiz = fileContent.match(
    new RegExp(`## ${quizNumber}\n(.*?)\n## ${quizNumber + 1}`, "s")
  )?.[1];
  return quiz;
}

/**
 * Gets the question of the quiz
 * @param {string} quiz - The quiz content
 * @returns {string} The question of the quiz
 */
function getQuizQuestion(quiz) {
  const question = quiz.match(
    new RegExp(`${QUESTION_MARKER}\n(.*?)\n(?=${NEXT_MARKER})`, "s")
  )?.[1];
  return question;
}

/**
 * Gets the options of the quiz
 * @param {string} quiz - The quiz content
 * @returns {Object[]} An array of `{ code: string, order: number }`
 */
function getQuizOptions(quiz) {
  const optionsString = quiz.trim().split(NEXT_MARKER)?.[2];
  const options = optionsString.match(/```[a-z]+\n.*?\n```/gms) ?? [];
  return options.reduce((acc, option, i) => {
    return [...acc, { code: option, order: i }];
  }, []);
}

function getQuizResults(quiz) {
  const resultsString = quiz.trim().split(NEXT_MARKER)?.[3];
  const results = resultsString.match(/(?<=```yml\n).*?(?=\n```)/ms)?.[0] ?? [];
  return results;
}

function parseResultsYaml(results) {
  const resultsYaml = yaml.load(results);
  return resultsYaml;
}

function parseQuiz() {
  const FILE = "./assets/quiz.md";
  const fileContent = fs.readFileSync(FILE, "utf8");
  const quizzes = [];
  for (let i = 1; i < NUM_QUIZZES + 1; i++) {
    const quizString = getQuizFromFileContent(fileContent, i);
    const results = getQuizResults(quizString);
    const resultsYaml = parseResultsYaml(results);

    const quiz = {
      question: getQuizQuestion(quizString),
      options: getQuizOptions(quizString),
      results: resultsYaml,
    };
    quizzes.push(quiz);
  }
  return quizzes;
}

module.exports = {
  getQuizFromFileContent,
  getQuizQuestion,
  getQuizOptions,
  getQuizResults,
  parseResultsYaml,
  parseQuiz,
};
