# Systems Knowledgebase

# Background

The systems knowledgebase plays an essential role in bringing systems language into the hands of systems analysts by organizing and managing system knowledge based on the theoretical foundations while enabling practical analysis through DSA tools.

While it technically isn’t a part of the language, it is an implementation mechanism for: 

1. Capturing and storing analysis results
2. Organizing system knowledge according to the formal definition
3. Enabling generation of models and implementations

# Function

1. Information about the SOI is captured using DSA tools and user interfaces
2. This information is stored in the knowledgebase according to the system definition schema
3. From the knowledgebase, various outputs can be generated including SysXML

It is a repository that stores system information in structured form and implements the formal system definition in a queryable form. It functions as a bridge which links theoretical constructs to real systems instance and connects formal models to practical analysis. It supports tools like BERT by providing data structure, enabling systematic analysis based on theory and supporting knowledge capture and reuse. 

"The system is analyzed following the procedures given in Chap. 6, in a top-down decomposition with the analytical engine capturing the results into the knowledgebase format... From there, it is possible to generate a systems dynamics-like simulation model and produce a human- (and machine-) readable sysXML specification" (pp.382-383)

"The knowledgebase is the complement of the systems analysis process. It is used to capture the details of a system's structures and functions as defined by the nature of systemness. Together, these components provide analysis and synthesis which is the objective of a whole systems approach to understanding." (p.389)

"The knowledgebase aggregates knowledge, structured by the definition of the system, such that it is available from any desired perspective." (p.366)

# Architecture

The current BERT prototype stores knowledgebase files in JSON format

Currently knowledgebase files are stored in JSON format.

Captures and stores relevant aspects and facts of a system from the analysis process into a schema for system structure. The schema of the knowledgebase is based on the mathematical definition of system.

"The knowledgebase, an artifactual system, can converse with a human mind (actually many minds) through a language facility to share knowledge about other systems. We use systems science to build a system of communication between machines and people that will, hopefully, enrich our totality of knowledge."

"The knowledgebase structure derives from the mathematical descriptions given in Chap. 4. A sample of knowledgebase tables and forms will be used to show how to capture the information about the system from the analysis." (p.359)

“The knowledgebase contents can be used to generate models at various levels of abstraction. And by models, here, we mean models suitable for computer simulation. The model can be as detailed as the analysis produced. If the intent was for deep scientific understanding, then the model might similarly be extremely detailed and require extensive computing resources to simulate. On the other hand, because of the way Eq. 4.1 through 4.3 are structured recursively it is possible to generate system models that are more abstract and, hence, useful for engineering or management purposes. A user of the modeling interface with the system knowledgebase need only indicate what level of abstraction is needed for a simulation. Since the transfer functions for any given module subsystem have been captured, it should be possible for the software to construct a simulation using those functions indicated in the level of the simulation requested.”

(Insert Images)

# Research Directions

## Graph Databases

- [And Now for Something Completely Different: Using OWL with Neo4j](https://neo4j.com/blog/using-owl-with-neo4j/)
- [Importing Ontologies](https://neo4j.com/labs/neosemantics/4.0/importing-ontologies/)
- [CypherGUI](https://www.notion.so/Systems-Knowledgebase-137e64cb6fe980caa79ff524b71d51a9?pvs=21)

Set Theory, Domain Semantics and XML

- [**Set Theory: the Method To Database Madness**](https://medium.com/basecs/set-theory-the-method-to-database-madness-5ec4b4f05d79)
- [A set theory based approach on applying domain semantics to XML-structures](https://ieeexplore.ieee.org/document/994070)
- [**A Set Theory Based Approach on Applying Domain Semantics to XML Structures**](https://www.computer.org/csdl/proceedings-article/hicss/2002/14350120/12OmNxYtuaq)
- [A Three Layer Meta Model for Specifying
Business Domain Semantics with XML](https://www.ambuehler.ethz.ch/cdstore/www2002/poster/176.pdf)
