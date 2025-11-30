# Specula Integration Guide

This project includes [Specula](https://github.com/specula-org/Specula) as a submodule in `vendor/specula`. Specula is a framework for synthesizing TLA+ specifications from source code.

## Prerequisites

- Python 3.8+
- Java 11+
- Maven
- An API Key for Anthropic (Claude) or OpenAI.

## Setup

1.  Initialize the submodule:
    ```bash
    git submodule update --init --recursive
    ```

2.  Run the setup script:
    ```bash
    cd vendor/specula
    bash scripts/setup.sh
    ```

3.  Export your API Key:
    ```bash
    export ANTHROPIC_API_KEY=your_api_key_here
    ```

## Running Specula

To generate a TLA+ specification for the accounting system:

1.  **Step 1: Code-to-Spec Translation**
    ```bash
    ./specula step1 ../../src/domain/entity.rs output/accounting/spec/step1/ --mode draft-based
    ```
    *Note: You may need to adjust the input file path or create a combined source file if you want to verify the entire system.*

2.  **Step 2: TLA+ Transformation**
    ```bash
    ./specula step2 output/accounting/spec/step1/corrected_spec/Entity.tla output/accounting/spec/step2/Entity.tla
    ```

3.  **Step 3: Verification**
    ```bash
    ./specula step3 output/accounting/spec/step2/Entity.tla output/accounting/spec/step3/
    ```

## Configuration

You can modify `vendor/specula/config.yaml` to change the model (e.g., use OpenAI instead of Anthropic) or adjust other settings.
