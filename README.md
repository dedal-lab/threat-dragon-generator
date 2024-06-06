# Introduction
Welcome to the Threat-Dragon-Generator! This open-source project is designed to streamline the process of creating inputs for OWASP Threat-Dragon from YAML files and to generate comprehensive reports in Excel format. One of the key philosophies behind this project is the "Documentation as Code" approach, ensuring that all threat modeling and reporting processes are automated, consistent, and seamlessly integrated into your development workflow.

OWASP Threat-Dragon is a popular tool for modeling and mitigating potential security threats in software development. However, manually creating inputs for this tool can be time-consuming and error-prone. Threat-Dragon-Generator automates this process, ensuring accuracy and efficiency. By treating documentation as code, we enable you to version control your threat models, integrate them with CI/CD pipelines, and maintain a single source of truth for your security documentation.

Whether you're a security professional, a developer, or part of a DevOps team, this tool can save you valuable time and effort. Our goal is to support the security community by providing a reliable and easy-to-use solution for threat modeling and reporting. We welcome contributions and feedback to continually improve the project. Thank you for choosing Threat-Dragon-Generator!

# Features
Threat-Dragon-Generator offers a comprehensive set of features designed to streamline and enhance your threat modeling and reporting processes. Here’s what you can do with this tool:

- <b>Description of Nodes and Flows in YAML:</b> Define your system's architecture using YAML files. Nodes represent various components within your system, while flows describe the interactions and data exchanges between these nodes. This structured approach ensures clarity and consistency across your threat models.

- <b>Description of Threats in YAML:</b> Specify potential threats directly within the YAML files. This feature allows you to systematically and consistently document threats, making it easier to identify, assess, and mitigate risks.

- <b>Description of Assets:</b> Document the critical assets within your system in the YAML files. Clearly defining your assets helps in understanding the value and importance of each component, leading to better protection strategies.

- <b>Description of Trust Boundaries:</b> Identify and describe trust boundaries within your system using YAML. Trust boundaries indicate where different levels of trust are required, highlighting areas that need closer security scrutiny and controls.

- <b>Generation of Child Diagrams from Parent Diagrams:</b> Simplify the management of complex systems by generating child diagrams that inherit properties from parent diagrams. This feature reduces redundancy and ensures that common elements are consistently represented across your threat models.

- <b>Generation of Excel Reports for Each Diagram:</b> Automatically generate comprehensive Excel reports for each diagram. These reports provide a detailed overview of your threat models, including nodes, flows, threats, assets, and trust boundaries, making it easier to analyze and document your security posture.

By leveraging these features, Threat-Dragon-Generator facilitates a robust "Documentation as Code" workflow, allowing you to manage your threat models and security documentation with the same rigor as your source code. This approach ensures consistency, accuracy, and integration with your development processes, ultimately enhancing your overall security strategy.

# Installation

To get started with Threat-Dragon-Generator, you need to compile and install the program from the source code available on GitHub. Follow the steps below to install the tool using Cargo, the Rust package manager.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- [Rust and Cargo](https://www.rust-lang.org/tools/install): Follow the instructions on the Rust website to install Rust and Cargo.

## Installation Steps

1. **Clone the Repository at the Specific Tag**

   Open your terminal and clone the Threat-Dragon-Generator repository from GitHub, checking out the tag `0.1.0` directly:

   ```sh
   git clone --branch 0.1.0 https://github.com/dedal-lab/threat-dragon-generator.git
   ```


2. **Navigate to the Project Directory**

    Change into the project directory:

    ```sh
    cd threat-dragon-generator
    ```
3. **Compile and Install**

    Use Cargo to compile and install the program:

    ```sh
    cargo install --path .
    ```

Once the installation is complete, you can start using Threat-Dragon-Generator to create inputs for OWASP Threat-Dragon and generate Excel reports.

# Usage

To use Threat-Dragon-Generator, you need to set up the required environment variables and ensure that your YAML configuration files and directory structure are correctly set. Here’s a step-by-step guide to get you started.

## Environment Variables

Set the following environment variables to specify the paths for the configuration and output files:

- **CONFIG_PATH**: Path to the config.yaml file containing general descriptions for threat modeling, trust boundaries, assets, and child diagram nodes.
- **THREAT_PATH**: Path to the threats.yaml file containing the list of threats.
- **DIAGRAM_PATH**: Path to the directory containing parent diagram YAML files.
- **OUTPUT_PATH**: Path to the directory where the Excel reports and OWASP Threat-Dragon input JSON files will be generated.

## Configuration Files

    1. config.yaml: Contains general descriptions for threat modeling (across all diagrams), trust boundaries (across all diagrams), assets (across all diagrams), and the selection of nodes for child diagrams (specifying the parent diagram).

    2. threats.yaml: Contains the list of threats (across all diagrams).

    3. diagrams: Directory containing the parent diagram YAML files. Each diagram should be in its own YAML file.

## Output Directory

The **output** directory will be used to store the generated Excel reports and the JSON input files for OWASP Threat-Dragon.

## Running the Tool

Once the environment variables are set and the configuration files are in place, you can run Threat-Dragon-Generator with the following command:

```sh
CONFIG_PATH="/path/to/your/project/config.yaml" \
THREAT_PATH="/path/to/your/project/threats.yaml" \
DIAGRAM_PATH="/path/to/your/project/diagrams" \
OUTPUT_PATH="/path/to/your/project/output" \
threat-dragon-generator
```

or settings env var before or in ~/.bashrc file and run threat-dragon-generator:
```sh
export BASE_PATH=/path/to/your/project
export CONFIG_PATH="$BASE_PATH/config.yaml"
export THREAT_PATH="$BASE_PATH/threats.yaml"
export DIAGRAM_PATH="$BASE_PATH/diagrams"
export OUTPUT_PATH="$BASE_PATH/output"
threat-dragon-generator
```

## Exemple Directory Structure
```lua
/path/to/your/project
│
├── config.yaml
├── threats.yaml
├── diagrams
│   ├── diagram1.yaml
│   ├── diagram2.yaml
│   └── ...
└── output
    ├── diagram1_report.xlsx
    ├── diagram1_child1_report.xlsx
    ├── diagram1_child2_report.xlsx
    ├── diagram2_report.xlsx
    ├── threat_dragon_input.json
    └── ...

```
By following these steps, you can effectively use Threat-Dragon-Generator to automate the creation of inputs for OWASP Threat-Dragon and generate detailed Excel reports for your threat models.

# Configuration
Threat-Dragon-Generator uses YAML configuration files to define the structure and details of your threat models. The key configuration files include config.yaml, threats.yaml, and individual diagram files located in the diagrams directory. Below are the details on how to configure each of these files.

## config.yaml

The config.yaml file contains general descriptions for threat modeling, trust boundaries, assets, and the selection of nodes for child diagrams. Here is an example structure:

```yaml
# config.yaml
threatDragonVersion: "2.2.0" # Version of OWASP Threat Dragon to use
title: "My threat modeling"
owner: "me"
description: "This is my description"
trustBoundaries: # Trust boundaries used in diagrams
  - name: Enablers
    description: "Enabler services provided"
    limitOfAccess: "Operational" # Limit of access Operational or Administration
    levelOfAuthorization: "Application data"
  - name: Enablers2
    description: "Enabler services provided"
    limitOfAccess: "Operational"
    levelOfAuthorization: "Application data"
assets: # Assets used in node flow
  - name: OpenApi
    description: "OpenApi messages"
diagrams: # Child diagrams
  - name: A2_Child_Data_Flow
    parent: A1_Diagram1 # title field in the parent diagram
    description: ""
    nodes: # list of nodes to display in child
      - MyProcess
      - RabbitMq
      - Prometheus
      - Minio
  - name: B2_Child_Data_Flow
    parent: B1_Diagram2
    description: ""
    nodes:
      - MyProcess
      - RabbitMq2

```

## threats.yaml

The threats.yaml file contains the list of threats that can be associated with nodes and flows in your diagrams. Here is an example structure:
```yaml
# threats.yaml
- title: "New STRIDE threat"
  status: "Open"       # Open, NotApplicable, Mitigated
  severity: "Medium"   # Low, Medium, High
  type: "DenialOfService"     # Spoofing, Tampering, Repudiation, InformationDisclosure, DenialOfService, ElevationOfPrivilege
  description: "Provide a description for this threat"
  mitigation: "Provide remediation for this threat or a reason if status is N/A"
  vector: "Attack vector"
- title: "New STRIDE threat 2"
  status: "Open"       # Open, NotApplicable, Mitigated
  severity: "Medium"   # Low, Medium, High
  type: "Spoofing"     # Spoofing, Tampering, Repudiation, InformationDisclosure, DenialOfService, ElevationOfPrivilege
  description: "Provide a description for this threat"
  mitigation: "Provide remediation for this threat or a reason if status is N/A"
  vector: "Attack vector"

```

## Diagram Files

Each diagram file in the diagrams directory represents a parent diagram and follows a specific structure. Below is an example of a parent diagram file:

```yaml
# diagrams/A1_Diagram1.yaml
title: "A1_Diagram1" # Diagram title
description: "Diagram1 STRIDE modeling"
nodes:
  - name: MyProcess # Process name
    type: process # Type of node, process or flow
    description: "Ecoute" # Node description
    outOfScope: false # If the node is out of scope or not
    trustLevel: "Operational" # The level of access required to access the entry point
    threats: # List of node threats (threats defined in threat.yaml)
      - "New STRIDE threat"
      - "New STRIDE threat 2"
  - name: PROM_PROC # Flow name
    type: flow # Type of node, process or flow
    source: Prometheus # Use field "name" defined inside the node source
    destination: MyProcess # Use field "name" defined inside the node destination
    description: "Publish message" # Flow description
    trustLevel: "Administration" # Level access of the entry point
    threats: # List of node threats (threats defined in threat.yaml)
      - "New STRIDE threat"

```

### Node Types

There are two types of nodes you can define in your diagram files:

1. Process Node

    ```yaml
    - name: MyProcess # Process name
      type: process # Type of node, process or flow
      description: "Ecoute" # Node description
      outOfScope: false # If the node is out of scope or not
      trustLevel: "Operational" # The level of access required to access the entry point
      threats: # List of node threats (threats defined in threat.yaml)
            - "New STRIDE threat"
            - "New STRIDE threat 2"

    ```

2. Flow Node

    ```yaml
    - name: PROM_PROC # Flow name
      type: flow # Type of node, process or flow
      source: Prometheus # Use field "name" defined inside the node source
      destination: MyProcess # Use field "name" defined inside the node destination
      description: "Publish message" # Flow description
      trustLevel: "Administration" # Level access of the entry point
      threats: # List of node threats (threats defined in threat.yaml)
            - "New STRIDE threat"

    ```
## Running the Tool

Once the environment variables are set and the configuration files are in place, you can run Threat-Dragon-Generator with the following command:

```sh
CONFIG_PATH="/path/to/your/project/config.yaml" \
THREAT_PATH="/path/to/your/project/threats.yaml" \
DIAGRAM_PATH="/path/to/your/project/diagrams" \
OUTPUT_PATH="/path/to/your/project/output" \
threat-dragon-generator
```
This command will read the configuration files, process the threat models, and generate the required Excel reports and OWASP Threat-Dragon input JSON files in the specified OUTPUT_PATH.
