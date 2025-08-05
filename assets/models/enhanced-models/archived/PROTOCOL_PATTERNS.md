# BERT Protocol Patterns - Algorithmic Boundary Control
*Based on Mobus's definition: "algorithms for letting flow across boundary in ordered fashion"*

## Core Protocol Structure

Protocols should describe:
1. **Trigger Condition** - What initiates the flow
2. **Sequential Steps** - Ordered algorithm
3. **Control Logic** - Decision points and conditions
4. **Reset Mechanism** - Return to ready state
5. **Regulation** - What modulates the rate/selectivity

## Biological Protocol Examples

### Import Protocols (Environment → System)

**Facilitated Diffusion**:
```
1. Substrate binds extracellular site → 
2. Conformational change triggered → 
3. Substrate released inside → 
4. Transporter resets
Rate: [Regulated by transporter density]
```

**Active Transport**:
```
1. ATP binds cytoplasmic domain → 
2. Substrate recognition → 
3. ATP hydrolysis drives conformational change → 
4. Substrate expelled → 
5. ADP+Pi released → 
6. Reset state
```

### Export Protocols (System → Environment)

**Vesicle Fusion**:
```
1. Vesicle docks at membrane → 
2. SNARE proteins zipper → 
3. Membrane fusion → 
4. Contents released → 
5. Membrane recycled
```

### Message Protocols (Information Transfer)

**Signal Transduction**:
```
1. Ligand binds receptor → 
2. Conformational change → 
3. Intracellular domain activated → 
4. Second messenger cascade → 
5. Signal amplification → 
6. Negative feedback terminates
```

## Technical System Protocol Examples

### Data Transfer:
```
1. Handshake initiated → 
2. Authentication verified → 
3. Data packets transmitted → 
4. Checksum validated → 
5. Acknowledgment sent → 
6. Connection closed
```

### Power Regulation:
```
1. Voltage monitored → 
2. If V > threshold, open circuit → 
3. If V < minimum, boost → 
4. Maintain within operational range → 
5. Log deviations
```

## Key Principles

1. **Unidirectional by Default** - Most interfaces allow flow in one direction only
2. **Conditional Passage** - "Only under the right conditions of the protocol"
3. **No Transformation** - Interfaces regulate flow, not transform substances
4. **Message + Mechanical** - Often combine information processing with physical control
5. **Filter Capability** - Some protocols selectively allow desired substances

## Template for New Protocols

```
1. [Initiation condition] → 
2. [Verification/recognition step] → 
3. [Main transfer action] → 
4. [Confirmation/validation] → 
5. [Reset to ready state]
[Regulation: What controls rate/selectivity]
```

This algorithmic approach makes protocols:
- **Executable** - Could be implemented in code
- **Verifiable** - Clear success/failure conditions
- **Educational** - Shows exact mechanism
- **Consistent** - Follows Mobus's theoretical framework