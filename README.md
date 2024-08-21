# AI-Powered CS:S Surfing Community Assistant

This project is an AI-powered assistant designed to help users in the Counter-Strike: Source (CS:S) surfing community find relevant information and get helpful advice. By leveraging state-of-the-art natural language processing and language generation models, the assistant can match user queries to the most relevant messages from a dataset, and then generate refined, informative responses.

## Table of Contents

- [Introduction](#introduction)
- [Key Features](#key-features)
- [Models and Skills](#models-and-skills)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)

## Introduction

This project is a tool that helps users find relevant information from a dataset of messages related to the Counter-Strike: Source (CS:S) surfing community. It uses advanced natural language processing techniques to match user queries to the most relevant messages, and then generates a refined, informative response based on the retrieved information.

## Key Features

- **Relevant Message Retrieval**: The assistant uses a sentence transformer model to calculate the cosine similarity between the user's query and the messages in the dataset, prioritizing more recent messages to provide the most up-to-date and relevant information.
- **Refined Response Generation**: The assistant utilizes a large language model (Mistral-7B) fine-tuned for text generation, along with a custom prompt template, to generate coherent and helpful responses to the user's queries.
- **Efficient Inference**: The language model is quantized using 4-bit precision, enabling efficient inference on resource-constrained devices without compromising the quality of the generated responses.

## Models and Skills

This project leverages the following models and skills:

- **Sentence Transformer**: The [paraphrase-MiniLM-L6-v2](https://www.sbert.net/docs/pretrained_models.html#paraphrase-models) model from the Sentence Transformers library is used for encoding the user's query and the messages in the dataset, enabling effective similarity calculation.
- **Causal Language Model**: The [Mistral-7B-v0.1](https://huggingface.co/mistralai/Mistral-7B-v0.1) model, pre-trained by Mistral AI, is used for generating the refined responses to the user's queries.
- **Natural Language Processing**: The project demonstrates skills in text preprocessing, embeddings, and cosine similarity calculation to find the most relevant messages.
- **Text Generation**: The assistant uses prompting and language model fine-tuning techniques to generate coherent and helpful responses to the user's queries.
- **Model Optimization**: The project showcases the ability to perform 4-bit quantization on the language model, enabling efficient inference on resource-constrained devices.

## Installation

To use this project, you'll need to have the following dependencies installed:

```bash
# Example installation commands
pip install -q pandas scikit-learn numpy sentence-transformers tqdm langchain langchain_community transformers bitsandbytes accelerate
```

## Usage

To use the project, simply run the provided code. The script will first check if the `messages.csv` file is available, and if not, it will prompt the user to upload the file. The code will then process the messages, generate embeddings, and set up the text generation pipeline.

To get a refined answer for a user's query, call the `generate_refined_answer` function with the user's query, the most relevant message, and the context:

```python
user_query = "how do i strafe?"
most_relevent_message = find_relevent_message(user_query, df)
context = """
The user is looking for help or advice related to the Counter-Strike: Source (CS:S) surfing community.
The user may have a specific question about surfing techniques, server settings, community events, or any other aspect of the CS:S surfing scene.
The relevant message retrieved from the CSV file, {most_relevent_message}, should provide some background information or a starting point for the user's query.
"""
refined_answer = generate_refined_answer(user_query, most_relevent_message, context)
print("Refined Answer:", refined_answer)
```

## Contributing

If you'd like to contribute to this project, please follow these steps:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Create a new Pull Request
