# Open Source Device Interface (OSDI)

## Specification - Working Draft

### Version 0.4

### January 2025

---

OSDI Copyright 2022-2025 SemiMod GmbH. All rights reserved.

---

## Contents

1. [Introduction](#1-introduction)
2. [General Overview](#2-general-overview)
   - 2.1 Exported Symbols
   - 2.2 Model Parameter Input
   - 2.3 Circuit Setup
   - 2.4 Evaluation
3. [Routines](#3-routines)
   - 3.1 access
   - 3.2 setup_model
   - 3.3 setup_instance
   - 3.4 eval
   - 3.5 load_noise
   - 3.6 load_residual_resist
   - 3.7 load_residual_react
   - 3.8 load_limit_rhs_resist
   - 3.9 load_limit_rhs_react
   - 3.10 load_spice_rhs_dc
   - 3.11 load_spice_rhs_tran
   - 3.12 load_jacobian_resist
   - 3.13 load_jacobian_react
   - 3.14 load_jacobian_tran
   - 3.15 given_flag_model
   - 3.16 given_flag_instance
   - 3.17 write_jacobian_array_resist
   - 3.18 write_jacobian_array_react
   - 3.19 load_jacobian_with_offset_resist
   - 3.20 load_jacobian_with_offset_react
4. [Callbacks](#4-callbacks)
   - 4.1 Log Messages
   - 4.2 Built-In $limit Functions
5. [Data Structures](#5-data-structures)
   - 5.1 OsdiDescriptor
   - 5.2 OsdiNoiseSource
   - 5.3 OsdiParamOpvar
   - 5.4 OsdiNode
   - 5.5 OsdiJacobianEntry
   - 5.6 OsdiNodePair
   - 5.7 OsdiInitInfo
   - 5.8 OsdiInitError
   - 5.9 OsdiInitErrorPayload
   - 5.10 OsdiSimInfo
   - 5.11 OsdiSimParas
   - 5.12 OsdiLimFunction
   - 5.13 OsdiNatureRef
   - 5.14 OsdiNature
   - 5.15 OsdiDiscipline
   - 5.16 OsdiAttribute
   - 5.17 OsdiAttributeValue
6. [Constants](#6-constants)
   - 6.1 Version number
   - 6.2 OsdiParamOpvar flags
   - 6.3 access function flags
   - 6.4 OsdiJacobianEntry flags
   - 6.5 eval argument flags
   - 6.6 eval return flags
   - 6.7 Log Level
   - 6.8 OsdiInitError error-codes
   - 6.9 Attribute type constants
   - 6.10 Nature reference constants
   - 6.11 Domain constants
7. [Verilog-A Standard Compliance](#7-verilog-a-standard-compliance)
   - 7.1 Hidden State
   - 7.2 limexp
8. [Files](#8-files)
   - 8.1 osdi.h

---

## 1 Introduction

All circuit simulators have their own unique interface for incorporating compact semiconductor device models. This slows down model integration, development, distribution and standardization. To remedy this situation, the Verilog-A language has been introduced as a means of having a unified description of compact models.

Most simulators are using the transpiler ADMS for incorporating Verilog-A models, yet ADMS's XML file-based transpilation approach has significant disadvantages compared to a true compilation approach with respect to execution time, compilation time and language standard support.

OSDI overcomes these disadvantages by defining a simulator-independent interface. Compact Verilog-A models are compiled to shared libraries that adhere to the OSDI interface as defined herein. Circuit simulators can then implement code that enables to interface with OSDI. The open-source Verilog-A compiler OpenVAF will serve as a back-end for generating shared libraries that adhere to OSDI from Verilog-A models.

Foremost, this positions OpenVAF and OSDI as the new open-source standard compiler solution for Verilog-A compact models, which in turn would be excellent for Verilog-A standardization. Furthermore, having a ready-to-use compiler will help circuit simulators obtain Verilog-A language support quicker, at reduced implementation and time effort.

```
                              OpenVAF                      Xyce
  Verilog-A    ───────────>                  ───────────>  Ngspice
   Model        Parsing, Code Optimization    shared OSDI  Other
                & Derivative Generation...    library      Simulators
```

---

## 2 General Overview

### 2.1 Exported Symbols

Each compiled shared library exports the following four symbols:

```c
extern uint32_t OSDI_VERSION_MAJOR;
extern uint32_t OSDI_VERSION_MINOR;
extern OsdiDescriptor OSDI_DESCRIPTORS[];
extern uint32_t OSDI_NUM_DESCRIPTORS;
```

The symbols `OSDI_VERSION_MAJOR` and `OSDI_VERSION_MINOR` indicate the OSDI version that the shared library was compiled with. Simulators that implement OSDI version X.Y must be able to load all shared libraries with `OSDI_VERSION_MAJOR = X` and `OSDI_VERSION_MINOR <= Y`. While OSDI is under development (Version X = 0) simulator implementations only have to support `OSDI_VERSION_MINOR = Y`.

`OSDI_DESCRIPTORS` is a list of device descriptors of length `OSDI_NUM_DESCRIPTORS`. A device descriptor is an instance of the `OsdiDescriptor` struct that contains all compiled information about a device model (Verilog-A module).

Compared to hard-coded models, OSDI models do not call simulator-specific functions (with a couple exceptions). Instead, `OsdiDescriptor` contains static metadata, enabling more flexibility for the simulator implementation. Compiler implementation is also simplified because static data is less complex to generate than executable code.

To store the metadata, OSDI defines several data structures. The static metadata is closely intertwined with the behavior of the compiled functions, hence both must be described together. Herein, first the overall working principle of the interface is roughly explained for each simulation stage. Thereafter, all data structures and routines are formally documented.

### 2.2 Model Parameter Input

During the first stage of simulation, the simulator reads the user configuration, creates the models/instances and populates these with parameters. After the correct device descriptor for each device is identified by the simulator, the size of the model and instance data can be obtained. The simulator must then allocate the required memory for storing this data.

A list of all model parameters is provided in the structure `param_opvar`. This list contains all static information about the parameters such as their names, types and so on. It does not contain the default values and bounds of the parameters, because these can depend on the value of other model parameters, simulator parameters and even temperature. Populating default values and checking parameter bounds are handled by OSDI during the circuit setup stage.

The values of parameters are stored within the model and instance data. These values can not directly be accessed by the circuit simulator since later simulation stages must know which parameters have been explicitly set, and which have remained at their default value. For accessing parameters, the `access` function must be used, it returns a pointer to a parameter and corresponding information. A set of flags is provided to this function that indicate how the simulator intends to use the pointer, so the OSDI model can update internal data correctly.

### 2.3 Circuit Setup

After the model/instance data is created and populated, the simulator must create the required nodes and respective jacobian entries. Furthermore, default values for all parameters that were not explicitly set during the parameter input stage are calculated. Finally, all calculations that only depend on parameter values and temperature are executed.

The `setup_model` routine handles model parameters (and dependent calculations). The `setup_instance` routine does the same for instance parameters. Furthermore, the `setup_instance` routine handles node collapsing and is therefore closely intertwined with the metadata that describes nodes and jacobian entries.

A list of all nodes and non-zero jacobian entries is provided by the `nodes` and `jacobian_entries` fields. The indices of these lists are very important as they are referring to jacobian entries and nodes elsewhere.

Multiple nodes may be collapsed into a single node, depending on the model and instance parameters. The `collapsible` field contains all pairs of nodes that can possibly be collapsed. Which node pairs are actually collapsed for a given instance is determined in the `setup_instance` routine, because node collapsing can not depend on the operating point. The results are stored within the instance data at offset `collapsed_offset`.

The simulator must use all this information to create a mapping from OSDI node IDs to circuit nodes. This mapping between node IDs must be stored at `node_mapping_offset` within the instance data. Using this mapping, all required matrix entries must be created by the simulator. The simulator is expected to write pointers to these entries into the instance data at `jacobian_ptr_resist_offset` and `react_ptr_off`.

Furthermore, the simulator must reserve for `num_states` values in the instance state. The indices of the reserved state entries must be stored in the instance data at `state_idx_off`. During evaluation these indices will be used to access the state in the `prev_state`/`next_state` pointers.

### 2.4 Evaluation

During the evaluation stage the operating point dependent Verilog-A code and its derivatives are evaluated. The results of these calculations are then loaded into the simulator RHS and jacobian entries. For accommodating all use cases, the evaluation stage is very flexible as OSDI supports different simulation modes and different simulator implementations.

For the most part these tasks are performed by the `eval` function. This function calculates the resistive residuals/jacobian entries and the reactive residuals/jacobian entries and stores them in the instance data. The reactive residuals/jacobian entries contains values that depend on time via (ddt), i.e. charges/capacitances.

This formulation is well suited for simulators that implement harmonic balance simulations like Xyce. These simulators can simply use the `load_residual_resist`, `load_residual_react`, `load_jacobian_resist` and `load_jacobian_react` functions to copy the residuals and jacobian entries from the instance data into the global simulator state.

However, traditional simulators are not directly compatible with this approach. The `load_jacobian_tran` function is available to support loading the reactive and resistive jacobian entries into a single matrix. For DC and AC simulations the `load_jacobian_resist` and `load_jacobian_react` functions can be used.

SPICE-like simulators in particular use the formulation of the NEWTON method shown in (2.2), instead of the usual formulation shown in (2.1). Therefore, the RHS of SPICE-like simulators contains further terms in addition to the residual. To support these simulators the `load_spice_rhs_dc` and `load_spice_rhs_tran` functions are available.

```
J(x_k - x_{k+1}) = F(x_k)     (2.1)
Jx_{k+1} = Jx_k - F(x_k)      (2.2)
```

---

## 3 Routines

Information that can not be exposed as static metadata can be accessed using the following routines. This comprises mainly compiled behavioral Verilog-A code and access to heterogeneous data inside the model and instance data.

All routines require a pointer to the model and/or instance data as arguments. These pointers must be allocated by the simulator with correct size, alignment and must be initialized with zeroed bytes. This can be achieved as follows:

```c
void* inst = calloc(1, descriptor->instance_size);
void* model = calloc(1, descriptor->model_size);
```

All functions that execute Verilog-A behavioral code must also be provided with the `void *handle` argument. This argument is used to invoke the `osdi_log` functions whenever Verilog-A emits messages. The contents of this pointer are entirely up to the simulator.

### 3.1 access

```c
void *access(void *inst, void *model, uint32_t id, uint32_t flags)
```

This function allows the simulator to read and write parameters as well as operating point variables. A pointer to the data corresponding to `param_opvar[id]` is returned and can be accessed by the simulator.

In order to work, the OSDI shared library must know how the simulator intends to use the returned argument. To that end bit flags are set in the `flags` argument. Reading the pointer never has an effect on the internal data and therefore requires no special flags. When `ACCESS_FLAG_SET` is set, the pointer can be written as well.

By default, the `access` function can be used to access model parameters. To access the corresponding instance parameter instead, `ACCESS_FLAG_INSTANCE` must be provided. If an instance parameter is not set for an instance, the parameter is copied from the model during `setup_instance`.

Note that none of these flags apply to operating point variables.

### 3.2 setup_model

```c
void setup_model(void *handle, void *model, OsdiSimParas *sim_params, OsdiInitInfo *res)
```

This function initializes all parameters that were not explicitly set using `access` and also performs a bounds check for all model parameters and instance parameters. Additionally, it executes all Verilog-A code that does not depend on:

- operating point
- analysis mode
- values of instance parameter
- `$param_given` with instance parameters as argument
- `$port_connected`
- `$temperature`
- `$simparam` and `$simparam$str`
- `@final_step` event

This function provides an instance of the `OsdiInitInfo` into the provided pointer. This struct contains a list of errors that occurred while checking the model parameters. Additionally, it contains any execution flags emitted by the behavioral Verilog-A code.

Whenever the model parameters change, this function must be called. Whenever this function is called for a model, the `setup_instance` routine must be called for all its instances as well. An `OsdiSimParas` struct is required by this function if any parameter default values invoke `$simparam`.

### 3.3 setup_instance

```c
void setup_instance(void *handle, void *inst, void *model,
                    double temperature, uint32_t num_terminals,
                    OsdiSimParas *sim_params, OsdiInitInfo *res)
```

This function initializes all instance parameters that have not been explicitly set, and also performs bounds checks for all instance parameters. Instance parameters that were not set for the instance but are set for the model are copied into the instance. The routine executes all Verilog-A code that was not executed in `setup_model` and does not depend on:

- operating point
- `@final_step` event
- `$simparam` and `$simparam$str`

This function requires a `OsdiSimParas` struct in case any parameter default values invoke `$simparam`. It puts an instance of the `OsdiInitInfo` into the provided pointer. This struct contains a list of errors that occurred while checking the model parameters. Flags emitted by the behavioral Verilog-A code are also contained in this struct.

A list of collapsible nodes within the instance data at `collapsed_offset` is also set-up by this function. When this function is called repeatedly (e.g. during a parameter sweep), the simulator must ensure that node collapsing is updated whenever collapsed changes. Alternatively, the simulator can disallow such sweeps producing an error whenever collapsed changes.

This function must be called whenever the model or instance parameters are changed.

### 3.4 eval

```c
uint32_t eval(void *handle, void *inst, void *model, OsdiSimInfo *info)
```

This function evaluates the remaining behavioral code in the Verilog-A analog block and puts the calculated jacobian entries and residuals in the instance data. Therefore, it encapsulates the major part of the Verilog-A code. This function can be configured to calculate multiple results that can be accessed by the simulator with separate functions:

- operating point variables - read with the `access` function
- noise source arguments - read with the `load_noise` function
- resistive residual - read with the `load_residual_resist`, `load_spice_rhs_dc` or `load_spice_rhs_tran` function or directly by using `resist_residual_off`
- reactive residual - read with the `load_residual_react` or `load_spice_rhs_tran` function or directly by using `react_residual_off`
- resistive jacobian entries - read with the `load_jacobian_resist`, `load_jacobian_tran`, or `write_jacobian_array_resist` function
- reactive jacobian entries - read with the `load_jacobian_react`, `load_jacobian_tran`, or `write_jacobian_array_react` function

This function does not calculate any numeric derivatives. Simulators that do not support a separate resistive and reactive RHS must calculate and load the numeric derivatives of the reactive residual themselves. This code will look roughly as follows:

```c
if (descriptor->nodes[i].is_reactive) {
   rhs[node_mapping[i]] += numeric_derivative(reactive_residual[i]);
}
```

### 3.5 load_noise

```c
void load_noise(void *inst, void *model, double freq, double *noise_dens)
```

This function is the primary function to be called for noise analysis. It generates the frequency dependent noise densities using the (operating point dependent) results of the `eval` function. The results are written into `noise_dens`. This pointer must point to a list of doubles with length `num_noise_src`. The element `noise_dens[i]` is set to the noise density that corresponds to the noise source described in `noise_sources[i]`.

**Note (Change from OSDI 0.3):** In OSDI 0.3, this function had an additional output parameter `ln_noise_dens` which was set to `log(noise_dens[i])` for each noise source. This parameter has been removed in OSDI 0.4. Simulators that require the logarithm of the noise density should compute it themselves from `noise_dens`.

### 3.6 load_residual_resist

```c
void load_residual_resist(void *inst, void* model, double *dst)
```

This function adds the resistive residuals calculated by `eval` function to the global simulator RHS. The results are read from the instance data and added to the value at the provided pointer location. Using the positions provided in the instance data at `node_mapping_offset`, the values are written.

### 3.7 load_residual_react

```c
void load_residual_react(void *inst, void* model, double *dst)
```

This function adds the reactive residuals calculated by `eval` function to the global simulator RHS. The results are read from the instance data and added to the value at the provided pointer location. Using the positions provided in the instance data at `node_mapping_offset`, the values are written.

### 3.8 load_limit_rhs_resist

```c
void load_limit_rhs_resist(void *inst, void* model, double *dst)
```

This function adds the resistive residuals calculated by `eval` function to the global simulator RHS. The results are read from the instance data and added to the value at the provided pointer location. Using the positions provided in the instance data at `node_mapping_offset`, the values are written.

### 3.9 load_limit_rhs_react

```c
void load_limit_rhs_react(void *inst, void* model, double *dst)
```

This function adds the reactive residuals calculated by `eval` function into the global simulator RHS. The results are read from the instance data and added to the value at the provided pointer location. Using the positions provided in the instance data at `node_mapping_offset`, the values are written.

### 3.10 load_spice_rhs_dc

```c
void load_spice_rhs_dc(void *inst, void* model, double *dst, double* prev_solve)
```

This function adds the resistive residuals calculated by `eval` function to the global simulator RHS. The results are read from the instance data and added to the value at the provided pointer location. Using the positions provided in the instance data at `node_mapping_offset`, the values is written.

Compared to `load_residual_resist`, this function adds the additional terms in (2.2) required for use within SPICE-like simulators. For calculating these terms, the `prev_solve` vector must be supplied to this function.

### 3.11 load_spice_rhs_tran

```c
void load_spice_rhs_tran(void *inst, void* model, double *dst,
                         double* prev_solve, double alpha)
```

This function adds the resistive residuals calculated by `eval` function to the global simulator RHS. The results are read from the instance data and added to the value at the provided pointer location. Using the positions provided in the instance data at `node_mapping_offset`, the values are written.

This function also adds the additional terms in (2.2) required for SPICE-like simulators. Compared to `load_spice_rhs_tran`, this function also considers the reactive jacobian entries while calculating the RHS. The reactive terms are multiplied with the `alpha` argument. `alpha` should be set to the factor that the reactive residuals are multiplied with during numeric differentiation. To calculate these terms the `prev_solve` vector must be supplied to this function.

This function does not add the reactive residuals to the simulator RHS. These terms must be numerically differentiated first which is simulator specific.

### 3.12 load_jacobian_resist

```c
void load_jacobian_resist(void *inst, void* model)
```

This function adds the resistive jacobian entries calculated by `eval` function into the global simulator jacobian. The results are read from the instance data and added to the value at the provided pointer at `jacobian_ptr_resist_offset`.

### 3.13 load_jacobian_react

```c
void load_jacobian_react(void *inst, void* model, double alpha)
```

This function adds the reactive jacobian entries calculated by `eval` function into the global simulator jacobian. The results are read from the instance data and added to the value at the provided pointers at `react_ptr_off`.

Every reactive jacobian entry is multiplied with `alpha`. For traditional simulators without separate resistive and reactive jacobian entries this argument can be set to `2*pi*f` to obtain the complex matrix during AC analysis.

### 3.14 load_jacobian_tran

```c
void load_jacobian_tran(void *inst, void* model, double alpha)
```

This function adds the resistive and reactive jacobian entries calculated by `eval` function into the global simulator jacobian. The results are read from the instance data and added to the value at the provided pointers at `jacobian_ptr_resist_offset`.

This function is intended for transient simulations in traditional simulators without separate resistive and reactive jacobian entries. Every reactive jacobian entry is multiplied with `alpha`. `alpha` should be set to the factor that the reactive residuals are multiplied with during numeric differentiation.

### 3.15 given_flag_model

```c
uint32_t given_flag_model(void *model, uint32_t id)
```

This function returns whether the model parameter with the given `id` was explicitly set by the user (returns non-zero) or left at its default value (returns zero). This is useful for simulators that need to query `$param_given` status without going through the `access` function.

### 3.16 given_flag_instance

```c
uint32_t given_flag_instance(void *inst, uint32_t id)
```

This function returns whether the instance parameter with the given `id` was explicitly set by the user (returns non-zero) or left at its default value (returns zero). This is useful for simulators that need to query `$param_given` status without going through the `access` function.

### 3.17 write_jacobian_array_resist

```c
void write_jacobian_array_resist(void *inst, void* model, double *destination)
```

This function writes the resistive jacobian entries calculated by the `eval` function to a contiguous array. The `destination` pointer must point to an array of `num_resistive_jacobian_entries` doubles. The entries are written in the same order as they appear in the `jacobian_entries` list (filtered to only include entries with the `JACOBIAN_ENTRY_RESIST` flag).

This function is useful for simulators that want to collect all jacobian entries into an array rather than scatter them to individual matrix locations via pointers.

### 3.18 write_jacobian_array_react

```c
void write_jacobian_array_react(void *inst, void* model, double *destination)
```

This function writes the reactive jacobian entries calculated by the `eval` function to a contiguous array. The `destination` pointer must point to an array of `num_reactive_jacobian_entries` doubles. The entries are written in the same order as they appear in the `jacobian_entries` list (filtered to only include entries with the `JACOBIAN_ENTRY_REACT` flag).

This function is useful for simulators that want to collect all jacobian entries into an array rather than scatter them to individual matrix locations via pointers.

### 3.19 load_jacobian_with_offset_resist

```c
void load_jacobian_with_offset_resist(void *inst, void* model, size_t offset)
```

This function adds the resistive jacobian entries calculated by `eval` function into the global simulator jacobian. Instead of using pointers stored in the instance data, this function uses the pointers stored at `jacobian_ptr_resist_offset` with an additional byte offset added to each pointer.

This is useful for simulators that need to load jacobian entries at a different offset than where the pointers originally point.

### 3.20 load_jacobian_with_offset_react

```c
void load_jacobian_with_offset_react(void *inst, void* model, size_t offset)
```

This function adds the reactive jacobian entries calculated by `eval` function into the global simulator jacobian. Instead of using pointers stored in the instance data, this function uses the pointers stored at `react_ptr_off` with an additional byte offset added to each pointer.

This is useful for simulators that need to load jacobian entries at a different offset than where the pointers originally point.

---

## 4 Callbacks

OSDI tries to avoid callbacks to simulator-specific functions. However, such callbacks can not be entirely avoided, hence additional symbols for these callbacks are exported by OSDI. These symbols are initiated by the simulator with appropriate function pointers. If the simulator does not populate one of these symbols undefined behavior will occur (unless specified otherwise). The compiler is allowed to omit generating these symbols if the appropriate functions are not used. An example for populating the `osdi_log` callback is shown below.

Every callback accepts a handle pointer. This pointer is passed by the simulator to any function that executes Verilog-A behavioral code. It is intended to allow the callbacks to access simulator-specific data.

```c
void **osdi_log_ = (void**) dlsym(lib, "osdi_log");
if (osdi_log_){
    *osdi_log_ = (void*) osdi_log_impl;
}
```

### 4.1 Log Messages

```c
extern void (*osdi_log)(void *handle, char* msg, uint32_t lvl);
```

Behavioral Verilog-A code can emit log messages with functions like `$strobe`. Handling of log messages is highly simulator specific, yet not performance critical. Therefore, OSDI delegates this functionality to a callback.

#### 4.1.1 osdi_log

```c
extern void (*osdi_log)(void *handle, char* msg, uint32_t lvl)
```

Formatting is closely intertwined with Verilog-A and needs to be handled by the compiler for proper standard compliance. The `osdi_log` callback receives the formatted message as an argument `msg`.

Furthermore, the `lvl` flag indicates what kind of message is emitted. This allows the simulator to format the message appropriately and emit when desired. The possible values of this argument are described in section 6.7.

**Note:** Ownership of the `msg` allocation is transferred to the simulator. It must free the `msg` to avoid memory leaks. However if `LOG_FMT_ERR` a string constant is passed to `osdi_log` that must not be freed.

A basic implementation of the function that writes each message immediately to stdout is shown below:

```c
extern void osdi_log(void *handle, char* msg, uint32_t lvl){
   switch(lvl & LOG_LVL_MASK){
      case LOG_LVL_DEBUG: printf("VA debug: "); break;
      case LOG_LVL_DISPLAY: printf("VA: "); break;
      case LOG_LVL_INFO: printf("VA info: "); break;
      case LOG_LVL_WARN: printf("VA warn: "); break;
      case LOG_LVL_ERR: printf("VA error: "); break;
      case LOG_LVL_FATAL: printf("VA fatal: "); break;
      default: printf("VA unknown message: "); break;
   }

   if (lvl & LOG_FMT_ERR){
      printf("FAILED TO FORMAT \"%s\"", msg);
   }else{
      printf("%s", msg);
   }
}
```

### 4.2 Built-In $limit Functions

```c
extern uint32_t OSDI_LIM_TABLE_LEN;
extern OsdiLimFunction OSDI_LIM_TABLE[];
```

Verilog-A allows using `$limit` to call simulator built-in functions such as `pnjlim` for improving the convergence properties of non-linear equations. These built-in functions are specified as a string and are therefore completely arbitrary. OSDI implements a dynamic look up table to support these functions.

These callbacks always have the following signature:

```c
double lim_callback(bool init, bool *limit, double old_val, double new_val, double arg1, double arg2);
```

The value returned by this function is used as the result for `$limit`. `init` is true if `INIT_LIM` is set, the function should return an initial value in that case. `limit` should be set to TRUE by the function if the function does not return `new_val`. `new_val` is the value of the limited branch in the current iteration. `old_val` is the value of the limited branch in the previous iteration. The remaining arguments are N arguments of type double. Here N is the number of arguments specified in `num_args`.

#### 4.2.1 OSDI_LIM_TABLE_LEN

```c
extern uint32_t OSDI_LIM_TABLE_LEN
```

This symbol is exported by OSDI shared libraries and specifies how many entries are present in `OSDI_LIM_TABLE`. If this symbol is missing from the loaded library, its value is assumed to be 0.

#### 4.2.2 OSDI_LIM_TABLE

```c
extern OsdiLimFunction OSDI_LIM_TABLE[]
```

A list of length `OSDI_LIM_TABLE` that contains information about every built-in `$limit` function used in the OSDI library. When the OSDI library is loaded, the simulator should set the `func_ptr` field for the table entries. If a function is used that is unknown to the simulator, a warning should be printed to the user and the pointer should be set to NULL. In this is the case the voltage/potential supplied to the function is returned without change.

---

## 5 Data Structures

### 5.1 OsdiDescriptor

```c
struct OsdiDescriptor {
  char *name;

  uint32_t num_nodes;
  uint32_t num_terminals;
  OsdiNode *nodes;

  uint32_t num_jacobian_entries;
  OsdiJacobianEntry *jacobian_entries;

  uint32_t num_collapsible;
  OsdiNodePair *collapsible;
  uint32_t collapsed_offset;

  OsdiNoiseSource *noise_sources;
  uint32_t num_noise_src;

  uint32_t num_params;
  uint32_t num_instance_params;
  uint32_t num_opvars;
  OsdiParamOpvar *param_opvar;

  uint32_t node_mapping_offset;
  uint32_t jacobian_ptr_resist_offset;

  uint32_t num_states;
  uint32_t state_idx_off;

  uint32_t bound_step_offset;

  uint32_t instance_size;
  uint32_t model_size;

  void *(*access)(void *inst, void *model, uint32_t id, uint32_t flags);

  void (*setup_model)(void *handle, void *model, OsdiSimParas *sim_params,
                      OsdiInitInfo *res);
  void (*setup_instance)(void *handle, void *inst, void *model,
                         double temperature, uint32_t num_terminals,
                         OsdiSimParas *sim_params, OsdiInitInfo *res);

  uint32_t (*eval)(void *handle, void *inst, void *model, OsdiSimInfo *info);
  void (*load_noise)(void *inst, void *model, double freq, double *noise_dens);
  void (*load_residual_resist)(void *inst, void* model, double *dst);
  void (*load_residual_react)(void *inst, void* model, double *dst);
  void (*load_limit_rhs_resist)(void *inst, void* model, double *dst);
  void (*load_limit_rhs_react)(void *inst, void* model, double *dst);
  void (*load_spice_rhs_dc)(void *inst, void* model, double *dst,
                            double* prev_solve);
  void (*load_spice_rhs_tran)(void *inst, void* model, double *dst,
                              double* prev_solve, double alpha);
  void (*load_jacobian_resist)(void *inst, void* model);
  void (*load_jacobian_react)(void *inst, void* model, double alpha);
  void (*load_jacobian_tran)(void *inst, void* model, double alpha);

  // New in OSDI 0.4
  uint32_t (*given_flag_model)(void *model, uint32_t id);
  uint32_t (*given_flag_instance)(void *inst, uint32_t id);
  uint32_t num_resistive_jacobian_entries;
  uint32_t num_reactive_jacobian_entries;
  void (*write_jacobian_array_resist)(void *inst, void* model, double* destination);
  void (*write_jacobian_array_react)(void *inst, void* model, double* destination);
  uint32_t num_inputs;
  OsdiNodePair* inputs;
  void (*load_jacobian_with_offset_resist)(void *inst, void* model, size_t offset);
  void (*load_jacobian_with_offset_react)(void *inst, void* model, size_t offset);
  OsdiNatureRef* unknown_nature;
  OsdiNatureRef* residual_nature;
}
```

This struct contains the entire OSDI API for one compiled Verilog-A module. Instances of this struct are provided in the `OSDI_DESCRIPTORS` list.

#### 5.1.1 name

```c
char *name
```

The name of the Verilog-A module.

#### 5.1.2 num_nodes

```c
uint32_t num_nodes
```

The total number of nodes used within the device descriptor. During simulation, the actual number of nodes may be lower due to node-collapsing.

#### 5.1.3 num_terminals

```c
uint32_t num_terminals
```

The number of nodes that are device terminals.

#### 5.1.4 nodes

```c
OsdiNode *nodes
```

A list of size `num_nodes` that contains metadata for each node used within the device descriptor. Each entry is an instance of `OsdiNode`. The node's index in this list is used to represent a node in all parts of the interface.

The first `num_terminals` entries are filled with the model's terminals in the same order as defined in Verilog-A source. A specific defined order does not need to be followed by the remaining entries.

#### 5.1.5 num_jacobian_entries

```c
uint32_t num_jacobian_entries
```

The number of non-zero jacobian entries.

#### 5.1.6 jacobian_entries

```c
OsdiJacobianEntry *jacobian_entries
```

A list with length `num_jacobian_entries` that contains the non-zero jacobian entries. Each entry contains an `OsdiJacobianEntry`. Other parts of the interface refer to jacobian entries via the indices in this list.

#### 5.1.7 num_collapsible

```c
uint32_t num_collapsible
```

The number of node pairs that can be collapsed, typically indicated by `V(x,y) <+ 0` in Verilog-A.

#### 5.1.8 collapsible

```c
OsdiNodePair *collapsible
```

A list with length `num_collapsible` that contains `OsdiNodePair` that are collapsible, typically indicated by `V(x,y) <+ 0` in Verilog-A. The `setup_instance` routine tells the simulator which of these pairs to collapse for a specific instance. Note: The `node_2` field can be set to `UINT32_MAX` to indicate a collapse into the global reference node.

#### 5.1.9 collapsed_offset

```c
uint32_t collapsed_offset
```

Provides the offset of the collapsed lists within the instance data in bytes. The collapsed list has length `num_collapsible` and contains bool elements.

`collapsed[i]` is set true if the collapsible node pair at `collapsible[i]` is collapsed for the given instance. The entries of collapsed are written in the `setup_instance` routine.

**Example:** You can access the collapsed list of the instance data `inst` as follows:
```c
bool *collapsed = (bool *) ((char *)inst) + collapsed_offset;
```

#### 5.1.10 num_noise_src

```c
uint32_t num_noise_src
```

The number of uncorrelated noise sources used in the model (`white_noise`, `flicker_noise`, `table_noise` and `table_noise_log` in Verilog-A).

#### 5.1.11 noise_sources

```c
OsdiNoiseSource *noise_sources
```

A list of all noise sources used within the device model with length `num_noise_src`.

#### 5.1.12 num_params

```c
uint32_t num_params
```

The number of Verilog-A model parameters.

#### 5.1.13 num_instance_params

```c
uint32_t num_instance_params
```

The number of Verilog-A parameters marked with the attribute `type="instance"`.

#### 5.1.14 num_opvars

```c
uint32_t num_opvars
```

The number of operating point variables (marked with `description` and `units` attribute).

#### 5.1.15 param_opvar

```c
OsdiParamOpvar *param_opvar
```

A list of metadata for each Verilog-A parameter with length `num_params + num_opvars`. Each element is an instance of the `OsdiParamOpvar` struct. The first `num_opvars` elements correspond to the model's operating point variables. The following `num_instance_params` elements correspond to the model's instance parameters. The remaining elements correspond to the model's parameters.

#### 5.1.16 node_mapping_offset

```c
uint32_t node_mapping_offset
```

The offset of the `node_mapping` within the instance data in bytes. This field can be used to calculate a pointer to the `node_mapping` as follows:

```c
uint32_t *node_mapping = (uint32_t *) ((char *)inst) + node_mapping_offset;
```

The `node_mapping` contains the global offset of each node within the solution/rhs vector. The values must be provided by the simulator before the `eval`, `load_residual_react`, `load_residual_resist`, `load_spice_rhs_dc` or `load_spice_rhs_tran` function are called. The offsets must be stored in the same order as the `nodes` list.

#### 5.1.17 jacobian_ptr_resist_offset

```c
uint32_t jacobian_ptr_resist_offset
```

The offset of the `jacobian_ptr_resist` array within the instance data in bytes. This field can be used to calculate a pointer to the `jacobian_ptr_resist` array as follows:

```c
double **jacobian_ptr_resist = (double **) ((char *)inst) + jacobian_ptr_resist_offset;
```

The `jacobian_ptr_resist` array contains pointers to resistive jacobian entries. These pointers must be provided by the simulator before the `load_jacobian_resist` / `load_jacobian_tran` functions are called. They must be stored in the same order as the `jacobian_entries` field.

The `load_jacobian_resist` and `load_jacobian_tran` functions use these pointers to store the calculated jacobian entries stored in the instance data.

#### 5.1.18 num_states

```c
uint32_t num_states
```

The number of states required for each instance. During initialization, the simulator must ensure that `num_states` doubles storage is available for each instance.

#### 5.1.19 state_idx_off

```c
uint32_t state_idx_off
```

The offset of the `state_idx` array within the instance data in bytes. This field can be used to calculate a pointer to the `state_idx` array as follows:

```c
uint32_t *state_idx = (uint32_t *) ((char *)inst) + state_idx_off);
```

The `eval` function will read/write `num_states` values from/to `prev_state`/`next_state` at the indices stored in this array. For example to read the value of the first state the `eval` function will perform the following:

```c
double state_1 = prev_state[state_idx[0]]
```

#### 5.1.20 bound_step_offset

```c
uint32_t bound_step_offset
```

The offset of the `bound_step` field within the instance data in bytes. At this field the `eval` function stores the minimum step size required for this instance. This step size is determined by the call to `$bound_step` with the smallest value.

For devices that never call `$bound_step` this value is set to `UINT32_MAX`. If a device calls `$bound_step` conditionally and the condition is not true for this instance then the value `+inf` is stored here.

#### 5.1.21 instance_size

```c
uint32_t instance_size
```

The size of the instance data in bytes.

#### 5.1.22 model_size

```c
uint32_t model_size
```

The size of the model data in bytes.

#### 5.1.23 access

```c
void *(*access)(void *inst, void *model, uint32_t id, uint32_t flags)
```

A function pointer to the implementation of the `access` function for this descriptor.

#### 5.1.24 setup_model

```c
void (*setup_model)(void *handle, void *model, OsdiSimParas *sim_params,
                    OsdiInitInfo *res)
```

A function pointer to the implementation of the `setup_model` function for this descriptor.

#### 5.1.25 setup_instance

```c
void (*setup_instance)(void *handle, void *inst, void *model,
                       double temperature, uint32_t num_terminals,
                       OsdiSimParas *sim_params, OsdiInitInfo *res)
```

A function pointer to the implementation of the `setup_instance` function for this descriptor.

#### 5.1.26 eval

```c
uint32_t (*eval)(void *handle, void *inst, void *model, OsdiSimInfo *info)
```

A function pointer to the implementation of the `eval` function for this descriptor.

#### 5.1.27 load_noise

```c
void (*load_noise)(void *inst, void *model, double freq, double *noise_dens)
```

A function pointer to the implementation of the `load_noise` function for this descriptor.

**Note (Change from OSDI 0.3):** In OSDI 0.3, this function had the signature:
```c
void (*load_noise)(void *inst, void *model, double freq, double *noise_dens, double *ln_noise_dens)
```
The `ln_noise_dens` parameter (which was set to `log(noise_dens[i])` for each noise source) has been removed in OSDI 0.4.

#### 5.1.28 load_residual_resist

```c
void (*load_residual_resist)(void *inst, void* model, double *dst)
```

A function pointer to the implementation of the `load_residual_resist` function for this descriptor.

#### 5.1.29 load_residual_react

```c
void (*load_residual_react)(void *inst, void* model, double *dst)
```

A function pointer to the implementation of the `load_residual_react` function for this descriptor.

#### 5.1.30 load_spice_rhs_dc

```c
void (*load_spice_rhs_dc)(void *inst, void* model, double *dst,
                          double* prev_solve)
```

A function pointer to the implementation of the `load_spice_rhs_dc` function for this descriptor.

#### 5.1.31 load_limit_rhs_resist

```c
void (*load_limit_rhs_resist)(void *inst, void* model, double *dst)
```

A function pointer to the implementation of the `load_limit_rhs_resist` function for this descriptor.

#### 5.1.32 load_limit_rhs_react

```c
void (*load_limit_rhs_react)(void *inst, void* model, double *dst)
```

A function pointer to the implementation of the `load_limit_rhs_react` function for this descriptor.

#### 5.1.33 load_spice_rhs_tran

```c
void (*load_spice_rhs_tran)(void *inst, void* model, double *dst,
                            double* prev_solve, double alpha)
```

A function pointer to the implementation of the `load_spice_rhs_tran` function for this descriptor.

#### 5.1.34 load_jacobian_resist

```c
void (*load_jacobian_resist)(void *inst, void* model)
```

A function pointer to the implementation of the `load_jacobian_resist` function for this descriptor.

#### 5.1.35 load_jacobian_react

```c
void (*load_jacobian_react)(void *inst, void* model, double alpha)
```

A function pointer to the implementation of the `load_jacobian_react` function for this descriptor.

#### 5.1.36 load_jacobian_tran

```c
void (*load_jacobian_tran)(void *inst, void* model, double alpha)
```

A function pointer to the implementation of the `load_jacobian_tran` function for this descriptor.

#### 5.1.37 given_flag_model (New in 0.4)

```c
uint32_t (*given_flag_model)(void *model, uint32_t id)
```

A function pointer to the implementation of the `given_flag_model` function for this descriptor. Returns non-zero if the parameter was explicitly set.

#### 5.1.38 given_flag_instance (New in 0.4)

```c
uint32_t (*given_flag_instance)(void *inst, uint32_t id)
```

A function pointer to the implementation of the `given_flag_instance` function for this descriptor. Returns non-zero if the parameter was explicitly set.

#### 5.1.39 num_resistive_jacobian_entries (New in 0.4)

```c
uint32_t num_resistive_jacobian_entries
```

The number of jacobian entries that have a resistive component (entries with `JACOBIAN_ENTRY_RESIST` flag set). This is used as the size for the array passed to `write_jacobian_array_resist`.

#### 5.1.40 num_reactive_jacobian_entries (New in 0.4)

```c
uint32_t num_reactive_jacobian_entries
```

The number of jacobian entries that have a reactive component (entries with `JACOBIAN_ENTRY_REACT` flag set). This is used as the size for the array passed to `write_jacobian_array_react`.

#### 5.1.41 write_jacobian_array_resist (New in 0.4)

```c
void (*write_jacobian_array_resist)(void *inst, void* model, double* destination)
```

A function pointer to the implementation of the `write_jacobian_array_resist` function for this descriptor.

#### 5.1.42 write_jacobian_array_react (New in 0.4)

```c
void (*write_jacobian_array_react)(void *inst, void* model, double* destination)
```

A function pointer to the implementation of the `write_jacobian_array_react` function for this descriptor.

#### 5.1.43 num_inputs (New in 0.4)

```c
uint32_t num_inputs
```

The number of input node pairs. This is typically used for small-signal analysis where specific inputs need to be identified.

#### 5.1.44 inputs (New in 0.4)

```c
OsdiNodePair* inputs
```

A list of `num_inputs` node pairs that represent the inputs to the device. This is typically used for small-signal analysis.

#### 5.1.45 load_jacobian_with_offset_resist (New in 0.4)

```c
void (*load_jacobian_with_offset_resist)(void *inst, void* model, size_t offset)
```

A function pointer to the implementation of the `load_jacobian_with_offset_resist` function for this descriptor.

#### 5.1.46 load_jacobian_with_offset_react (New in 0.4)

```c
void (*load_jacobian_with_offset_react)(void *inst, void* model, size_t offset)
```

A function pointer to the implementation of the `load_jacobian_with_offset_react` function for this descriptor.

#### 5.1.47 unknown_nature (New in 0.4)

```c
OsdiNatureRef* unknown_nature
```

A list of `num_nodes` nature references indicating the nature of the unknown quantity for each node. This provides information about the physical nature (voltage, current, etc.) of each unknown in the system.

#### 5.1.48 residual_nature (New in 0.4)

```c
OsdiNatureRef* residual_nature
```

A list of `num_nodes` nature references indicating the nature of the residual for each node. This provides information about the physical nature (current, voltage, etc.) of each residual in the system.

### 5.2 OsdiNoiseSource

```c
struct OsdiNoiseSource {
  char *name;
  OsdiNodePair nodes;
}
```

This struct describes a noise source. A single instance corresponds to a single call to one of the following Verilog-A functions:

- `white_noise`
- `flicker_noise`
- `table_noise`
- `table_noise_log`

Correlated noise sources are not directly supported at the moment. Instead, a correlation network should be used. Therefore, every noise source can only be used in single Verilog-A contribute statement and is only connected to a single pair of nodes.

#### 5.2.1 name

```c
char *name
```

This field contains the name of the noise source if given as a last argument in Verilog-A. If no name was provided this field contains a NULL pointer.

#### 5.2.2 nodes

```c
OsdiNodePair nodes
```

The pair of nodes that the noise source is connected to. `node_1` is the node with a positive contribution.

### 5.3 OsdiParamOpvar

```c
struct OsdiParamOpvar {
  char **name;
  uint32_t num_alias;
  char *description;
  char *units;
  uint32_t flags;
  uint32_t len;
}
```

Metadata that describes model parameters, instance parameters and operating point variables.

#### 5.3.1 name

```c
char **name
```

A list of identifiers with `1 + num_alias` entries. Its first entry corresponds to the canonical identifier while the remaining `num_alias` entries are aliases.

#### 5.3.2 num_alias

```c
uint32_t num_alias
```

The number of additional identifiers (aliases) that can be used for this parameter besides the canonical identifier.

#### 5.3.3 description

```c
char *description
```

Description of the parameter/op variable as given by the (standardized) Verilog-A `description` attribute.

#### 5.3.4 units

```c
char *units
```

Units of the parameter/op variable as given by the (standardized) Verilog-A `units` attribute.

#### 5.3.5 flags

```c
uint32_t flags
```

This field stores binary encoded information such as the data type. The bit patterns are documented in section 6.2.

#### 5.3.6 len

```c
uint32_t len
```

This field is used to encode array types. The value of this field is 0 for scalar types. For arrays, it is set to the total length of the array: For arrays with multiple dimensions their lengths are multiplied.

**Example:** A Verilog-A array `[10][5][2]` has a total length of 100.

### 5.4 OsdiNode

```c
struct OsdiNode {
  char *name;
  char *units;
  char *residual_units;
  uint32_t resist_residual_off;
  uint32_t react_residual_off;
  uint32_t resist_limit_rhs_off;
  uint32_t react_limit_rhs_off;
  bool is_flow;
}
```

This struct contains metadata describing a node used within OSDI.

#### 5.4.1 name

```c
char *name
```

The name of the node. For net potentials this corresponds to the name of the Verilog-A net.

#### 5.4.2 units

```c
char *units
```

The units of the unknown quantity this node represents. The contents of this field is derived from the Verilog-A discipline/nature system and depend on what kind of unknown the node represents. For net potentials, this string corresponds to the contents of the `units` attribute of the `potential_nature`.

#### 5.4.3 residual_units

```c
char *residual_units
```

The units of the residual associated with the unknown this node represents. The contents of this field is derived from the Verilog-A discipline/nature system and depend on what kind of unknown the node represents. For net potentials this string corresponds to the contents of the `units` attribute of the `flow_nature`.

#### 5.4.4 resist_residual_off

```c
uint32_t resist_residual_off
```

The offset (in bytes) of this nodes' resistive residual within instance data. If this node has no resistive residual this value is set to `UINT32_MAX`. Simulators usually do not need to access this data directly. Instead, functions like `load_residual_resist`, `load_spice_rhs_dc` and `load_spice_rhs_tran` should be used.

#### 5.4.5 react_residual_off

```c
uint32_t react_residual_off
```

The offset (in bytes) of this nodes' reactive residual within instance data. If this node has no reactive residual this value is set to `UINT32_MAX`. Simulators (like SPICE) that require that every model implementation calculates numeric derivatives manually can read the charges using this offset. Otherwise, `load_residual_react` should be used.

#### 5.4.6 resist_limit_rhs_off

```c
uint32_t resist_limit_rhs_off
```

The offset (in bytes) of this nodes' resistive `$limit` residual within instance data. If this node has no resistive `$limit` residual this value is set to `UINT32_MAX`. Simulators usually do not need to access this data directly. Instead, functions like `load_limit_rhs_resist`, `load_spice_rhs_dc` and `load_spice_rhs_tran` should be used.

#### 5.4.7 react_limit_rhs_off

```c
uint32_t react_limit_rhs_off
```

The offset (in bytes) of this nodes' reactive `$limit` residual within instance data. If this node has no reactive `$limit` residual this value is set to `UINT32_MAX`. Simulators usually do not need to access this data directly. Instead, functions like `load_limit_rhs_react`, `load_spice_rhs_dc` and `load_spice_rhs_tran` should be used.

#### 5.4.8 is_flow

```c
bool is_flow
```

In OSDI nodes refer to any simulator unknown. Usually equations are formulated in terms of current-controlled current sources. In those cases all unknowns are node potentials (hence the name). However, if other sources are used as unknowns for currents (flow) are required. In those cases this field is set to true.

### 5.5 OsdiJacobianEntry

```c
struct OsdiJacobianEntry {
  OsdiNodePair nodes;
  uint32_t react_ptr_off;
  uint32_t flags;
}
```

This struct contains metadata describing a non-zero jacobian entry used within OSDI.

#### 5.5.1 nodes

```c
OsdiNodePair nodes
```

The nodes of this matrix entry. The first node corresponds to the row, the second node corresponds to the column in the jacobian.

#### 5.5.2 react_ptr_off

```c
uint32_t react_ptr_off
```

The offset of the pointer to the reactive value of this jacobian entry within the instance data in bytes. This field is set to `UINT32_MAX` if this jacobian entry has no reactive component. Otherwise, the field can be used to calculate a pointer as follows:

```c
double **jacobian_ptr_react = (double **) (((char *)inst) + react_ptr_off);
```

The pointer stored here must be provided by the simulator before the `load_jacobian_react` / `load_jacobian_tran` functions are called. The `load_jacobian_react` function uses these pointers to store the calculated reactive jacobian entries stored in the instance data.

#### 5.5.3 flags

```c
uint32_t flags
```

Various properties about the jacobian entry encoded as bit_flags. These flags are documented in section 6.4.

### 5.6 OsdiNodePair

```c
struct OsdiNodePair {
  uint32_t node_1;
  uint32_t node_2;
}
```

This struct contains a pair of nodes. The nodes are referenced by their index in the `nodes` list.

### 5.7 OsdiInitInfo

```c
struct OsdiInitInfo {
  uint32_t flags;
  uint32_t num_errors;
  OsdiInitError *errors;
}
```

This struct is returned by the `setup_model` and `setup_instance` functions. It provides feedback about the setup process to the simulator.

#### 5.7.1 flags

```c
uint32_t flags
```

Status flags that can be set by executed Verilog-A code to control the flow of the simulation. These flags are documented in section 6.6.

#### 5.7.2 num_errors

```c
uint32_t num_errors
```

The number of errors that occurred during setup.

#### 5.7.3 errors

```c
OsdiInitError *errors
```

A list of errors that occurred during setup. This list has a length of `num_errors`. The memory that is pointed to by this field is allocated by the setup routine with `calloc` (if `num_errors != 0`). The caller must free this memory to avoid memory leaks.

### 5.8 OsdiInitError

```c
struct OsdiInitError {
  uint32_t code;
  OsdiInitErrorPayload payload;
}
```

If errors (usually caused by invalid user input) occur during the `setup_model` or `setup_instance` function, a list of `OsdiInitError` instances is allocated and returned in the `OsdiInitInfo` struct.

#### 5.8.1 code

```c
uint32_t code
```

This field contains an integer that represents the type of error. The error codes are documented in section 6.8.

#### 5.8.2 payload

```c
OsdiInitErrorPayload payload
```

Additional information associated with the occurred error.

### 5.9 OsdiInitErrorPayload

```c
union OsdiInitErrorPayload {
  uint32_t parameter_id;
}
```

This union contains additional information about an `OsdiInitError`. Which variant is contained within this union depends on the error code in the `code` field.

#### 5.9.1 parameter_id

```c
uint32_t parameter_id
```

This variant is used for errors associated with a parameter. The parameter is represented by its index with the `param_opvar` list.

### 5.10 OsdiSimInfo

```c
struct OsdiSimInfo {
   OsdiSimParas paras;
   double abstime;
   double *prev_solve;
   double *prev_state;
   double *next_state;
   uint32_t flags;
}
```

This struct contains various information about the simulation required by the `eval` function.

#### 5.10.1 paras

```c
OsdiSimParas paras
```

This field requires a pointer to an instance of `OsdiSimParas` struct. This struct contains the simulation parameters returned by `$simparam` and `$simparam$str`.

#### 5.10.2 abstime

```c
double abstime
```

The value returned by the `$abstime` function. It should contain the current time (in seconds) during large signal simulations. During DC and small signal simulations this field should be set to 0.

#### 5.10.3 prev_solve

```c
double *prev_solve
```

This field must point to the solution of the previous iteration (or initial values). The value of the unknown of each node will be read from this pointer. To that end the mapping stored at `node_mapping_offset` within the instance data is used.

#### 5.10.4 prev_state

```c
double *prev_state
```

This field must point to data that can be used for internal state of the model. The main use case is the implementation of `$limit` at the moment. Internal state can not be stored within the instance data, because large signal analysis might discard a timestep. In that case the internal state must be reset to the previous time step. The various internal state will be read from this pointer between `states[state_off]` and `states[state_off + num_states]`. Here `state_off` is an argument to the `eval` function.

#### 5.10.5 next_state

```c
double *next_state
```

This pointer is used the same as the `prev_state` field except that internal state is written instead of read. For simulators where the internal state of the current and the next iteration are stored at the same place `next_state=prev_state` can be used.

#### 5.10.6 flags

```c
uint32_t flags
```

This argument indicates which values must be calculated by the `eval` function. It contains bitflags that each enable some calculations listed in the documentation for the `eval` function. These flags are documented in section 6.5.

### 5.11 OsdiSimParas

```c
struct OsdiSimParas {
  char **names;
  double *vals;
  char **names_str;
  char **vals_str;
}
```

This struct contains lists of known simulation parameters that can be returned by the `$simparam` and `$simparam$str` Verilog-A function. What values are supported is up to the simulator. Primarily simulators should focus on supporting the parameters listed in Table 9-27 of the Verilog-AMS language reference manual (2.4.0). The `gmin` parameter in particular are commonly used by compact models. Additionally, the CMC recommends the parameter `minr` as a minimal resistance between two nodes, for smaller values the nodes are collapsed.

#### 5.11.1 names

```c
char **names
```

A list of names of all real-valued simulation parameters. The last entry in this list must contain a NULL pointer to indicate the end of the list.

#### 5.11.2 vals

```c
double *vals
```

A list of the values that correspond to the simulation parameters in `names` at the same index.

#### 5.11.3 names_str

```c
char **names_str
```

A list of names of all string-valued simulation parameters. The last entry in this list must contain a NULL pointer to indicate the end of the list.

#### 5.11.4 vals_str

```c
char **vals_str
```

A list of the values that correspond to the simulation parameters in `names_str` at the same index.

### 5.12 OsdiLimFunction

```c
struct OsdiLimFunction {
  char *name;
  uint32_t num_args;
  void *func_ptr;
}
```

This struct contains information about a built-in `$limit` function. The simulator shall use that information to find the implementation of this function and write a pointer to this function into the `func_ptr` field.

#### 5.12.1 name

```c
char *name
```

The name of the function specified in Verilog-A.

#### 5.12.2 num_args

```c
uint32_t num_args
```

The number of additional double arguments received by this function. Because the compiler has no way to know the correct number of arguments, it is imperative that the simulator checks that the number of arguments is correct. In fact OSDI even allows a function with the same name used with multiple different numbers of arguments. This allows simulators to support optional arguments for `$limit` functions.

#### 5.12.3 func_ptr

```c
void *func_ptr
```

This field contains a pointer to the implementation of the function. The simulator must store this pointer here when it loads the library. If the simulator does not have an implementation of the function, this field should be set to NULL.

### 5.13 OsdiNatureRef (New in 0.4)

```c
struct OsdiNatureRef {
  uint32_t ref_type;
  uint32_t index;
}
```

This struct contains a reference to a nature, either directly or via a discipline. This is used in OSDI 0.4 to provide information about the physical nature of unknowns and residuals.

#### 5.13.1 ref_type

```c
uint32_t ref_type
```

The type of reference. See section 6.10 for the possible values:
- `NATREF_NONE` (0): No nature reference
- `NATREF_NATURE` (1): Direct reference to a nature
- `NATREF_DISCIPLINE_FLOW` (2): Reference to a discipline's flow nature
- `NATREF_DISCIPLINE_POTENTIAL` (3): Reference to a discipline's potential nature

#### 5.13.2 index

```c
uint32_t index
```

The index of the referenced nature or discipline in the appropriate list.

### 5.14 OsdiNature (New in 0.4)

```c
struct OsdiNature {
  char *name;
  uint32_t parent_type;
  uint32_t parent;
  uint32_t ddt;
  uint32_t idt;
  uint32_t attr_start;
  uint32_t num_attr;
}
```

This struct describes a Verilog-A nature definition. Natures define physical quantities like voltage, current, temperature, etc.

#### 5.14.1 name

```c
char *name
```

The name of the nature (e.g., "Voltage", "Current").

#### 5.14.2 parent_type

```c
uint32_t parent_type
```

The type of reference to the parent nature. Uses the same constants as `OsdiNatureRef.ref_type`.

#### 5.14.3 parent

```c
uint32_t parent
```

The index of the parent nature, if this nature inherits from another.

#### 5.14.4 ddt

```c
uint32_t ddt
```

Index of the nature that results from taking the time derivative of this nature (e.g., charge -> current).

#### 5.14.5 idt

```c
uint32_t idt
```

Index of the nature that results from taking the time integral of this nature (e.g., current -> charge).

#### 5.14.6 attr_start

```c
uint32_t attr_start
```

The starting index of this nature's attributes in the global attribute list.

#### 5.14.7 num_attr

```c
uint32_t num_attr
```

The number of attributes for this nature.

### 5.15 OsdiDiscipline (New in 0.4)

```c
struct OsdiDiscipline {
  char *name;
  uint32_t flow;
  uint32_t potential;
  uint32_t domain;
  uint32_t attr_start;
  uint32_t num_flow_attr;
  uint32_t num_potential_attr;
  uint32_t num_user_attr;
}
```

This struct describes a Verilog-A discipline definition. Disciplines pair a potential nature with a flow nature (e.g., electrical pairs voltage with current).

#### 5.15.1 name

```c
char *name
```

The name of the discipline (e.g., "electrical", "thermal").

#### 5.15.2 flow

```c
uint32_t flow
```

Index of the flow nature (e.g., current for electrical).

#### 5.15.3 potential

```c
uint32_t potential
```

Index of the potential nature (e.g., voltage for electrical).

#### 5.15.4 domain

```c
uint32_t domain
```

The domain of the discipline. See section 6.11 for possible values:
- `DOMAIN_NOT_GIVEN` (0)
- `DOMAIN_DISCRETE` (1)
- `DOMAIN_CONTINUOUS` (2)

#### 5.15.5 attr_start

```c
uint32_t attr_start
```

The starting index of this discipline's attributes in the global attribute list.

#### 5.15.6 num_flow_attr

```c
uint32_t num_flow_attr
```

The number of flow nature attributes.

#### 5.15.7 num_potential_attr

```c
uint32_t num_potential_attr
```

The number of potential nature attributes.

#### 5.15.8 num_user_attr

```c
uint32_t num_user_attr
```

The number of user-defined attributes.

### 5.16 OsdiAttribute (New in 0.4)

```c
struct OsdiAttribute {
  char *name;
  uint32_t value_type;
  OsdiAttributeValue value;
}
```

This struct describes an attribute for a nature or discipline.

#### 5.16.1 name

```c
char *name
```

The name of the attribute.

#### 5.16.2 value_type

```c
uint32_t value_type
```

The type of the attribute value. See section 6.9 for possible values:
- `ATTR_TYPE_STR` (0): String value
- `ATTR_TYPE_INT` (1): Integer value
- `ATTR_TYPE_REAL` (2): Real (double) value

#### 5.16.3 value

```c
OsdiAttributeValue value
```

The value of the attribute.

### 5.17 OsdiAttributeValue (New in 0.4)

```c
union OsdiAttributeValue {
  char* string;
  int32_t integer;
  double real;
}
```

This union holds the value of an attribute. Which variant is active depends on the `value_type` field in `OsdiAttribute`.

#### 5.17.1 string

```c
char* string
```

String value, used when `value_type` is `ATTR_TYPE_STR`.

#### 5.17.2 integer

```c
int32_t integer
```

Integer value, used when `value_type` is `ATTR_TYPE_INT`.

#### 5.17.3 real

```c
double real
```

Real value, used when `value_type` is `ATTR_TYPE_REAL`.

---

## 6 Constants

### 6.1 Version number

```c
#define OSDI_VERSION_MAJOR_CURR 0
#define OSDI_VERSION_MINOR_CURR 4
```

The current OSDI version number.

Simulators that implement the current OSDI version 0.4 must be able to load all shared libraries with `OSDI_VERSION_MAJOR = 0` and `OSDI_VERSION_MINOR = 4`.

#### 6.1.1 OSDI_VERSION_MAJOR_CURR

```c
#define OSDI_VERSION_MAJOR_CURR 0
```

Current major version of OSDI.

#### 6.1.2 OSDI_VERSION_MINOR_CURR

```c
#define OSDI_VERSION_MINOR_CURR 4
```

Current minor version of OSDI.

### 6.2 OsdiParamOpvar flags

```c
#define PARA_TY_MASK 3
#define PARA_TY_REAL 0
#define PARA_TY_INT 1
#define PARA_TY_STR 2
#define PARA_KIND_MASK (3 << 30)
#define PARA_KIND_MODEL (0 << 30)
#define PARA_KIND_INST (1 << 30)
#define PARA_KIND_OPVAR (2 << 30)
```

Various properties about `OsdiParamOpvar` are encoded in its `flags` field as bit patterns. These bit patterns are documented herein.

#### 6.2.1 PARA_TY_MASK

```c
#define PARA_TY_MASK 3
```

A bitmask that allows obtaining basic type of this parameter/op variable. `flags & OSDI_TY_MASK` will yield a number that corresponds to one of the basic types (`PARA_TY_*`).

A basic type can never be an array but rather indicates the type of the scalar elements. Whether a type is an array is encoded with the `len` field instead.

#### 6.2.2 PARA_TY_REAL

```c
#define PARA_TY_REAL 0
```

A basic type (see `PARA_TY_MASK`) that corresponds to the `real` datatype in Verilog-A. Parameters and op variables with this type store their data as `double`.

#### 6.2.3 PARA_TY_INT

```c
#define PARA_TY_INT 1
```

A basic type (see `PARA_TY_MASK`) that corresponds to the `integer` datatype in Verilog-A. Parameters and op variables with this type store their data as `int32_t`.

#### 6.2.4 PARA_TY_STR

```c
#define PARA_TY_STR 2
```

A basic type (see `PARA_TY_MASK`) that corresponds to the `string` datatype in Verilog-A. Parameters and op variables with this type store their data as `char *` (null terminated UTF-8 strings).

#### 6.2.5 PARA_KIND_MASK

```c
#define PARA_KIND_MASK (3 << 30)
```

Bitmask that allows obtaining the kind of object is represented by an `OsdiParamOpvar` instance. `flags & PARA_KIND_MASK` will yield a number that corresponds to one of the constants (`PARA_KIND*`) defined below.

#### 6.2.6 PARA_KIND_MODEL

```c
#define PARA_KIND_MODEL (0 << 30)
```

Represents an `OsdiParamOpvar` kind (see `PARA_KIND_MASK`). This bit pattern is used for model parameters. All Verilog-A parameters are model parameters unless specified otherwise.

#### 6.2.7 PARA_KIND_INST

```c
#define PARA_KIND_INST (1 << 30)
```

Represents an `OsdiParamOpvar` kind (see `PARA_KIND_MASK`). This bit pattern is used for instance parameters. Verilog-A parameters must be marked with the `type="instance"` attribute to be treated as instance parameters.

#### 6.2.8 PARA_KIND_OPVAR

```c
#define PARA_KIND_OPVAR (2 << 30)
```

Represents an `OsdiParamOpvar` kind (see `PARA_KIND_MASK`). This bit pattern is used for operating point variables. Operating point variables are Verilog-A variables marked with the `description` and/or the `units` attribute. Their values are calculated by the `eval` function when `CALC_OP` is set in `flags` argument of the `eval` function.

### 6.3 access function flags

```c
#define ACCESS_FLAG_READ 0
#define ACCESS_FLAG_SET 1
#define ACCESS_FLAG_INSTANCE 4
```

The `access` function must know how the pointer it returns is used. To that end the simulator must provide a set of bit flags that indicate the usage.

#### 6.3.1 ACCESS_FLAG_READ

```c
#define ACCESS_FLAG_READ 0
```

This flag indicates that the returned pointer will be read. Note: Reading the pointer is always allowed, and therefore this flag is simply 0. It's only provided to improve the readability of simulator implementations.

#### 6.3.2 ACCESS_FLAG_SET

```c
#define ACCESS_FLAG_SET 1
```

This flag indicates that a value will be written to the returned pointer.

#### 6.3.3 ACCESS_FLAG_INSTANCE

```c
#define ACCESS_FLAG_INSTANCE 4
```

The `access` function always returns a pointer to model data for parameters. This allows the user to set a model default value for an instance parameter. When the `ACCESS_FLAG_INSTANCE` is set, a pointer to the instance value of the parameter is returned instead.

### 6.4 OsdiJacobianEntry flags

```c
#define JACOBIAN_ENTRY_RESIST_CONST 1
#define JACOBIAN_ENTRY_REACT_CONST 2
#define JACOBIAN_ENTRY_RESIST 4
#define JACOBIAN_ENTRY_REACT 8
```

The jacobian entries are of central importance to the simulator. Therefore, various additional properties about each entry are exposed by flags.

#### 6.4.1 JACOBIAN_ENTRY_RESIST_CONST

```c
#define JACOBIAN_ENTRY_RESIST_CONST 1
```

This flag indicates that the resistive component of this jacobian entry is constant. In this context a jacobian entry is regarded as constant if it's always independent of the operating point.

#### 6.4.2 JACOBIAN_ENTRY_REACT_CONST

```c
#define JACOBIAN_ENTRY_REACT_CONST 2
```

This flag indicates that the reactive component of this jacobian entry is constant. In this context a jacobian entry is regarded as constant if it's always independent of the operating point.

#### 6.4.3 JACOBIAN_ENTRY_RESIST

```c
#define JACOBIAN_ENTRY_RESIST 4
```

This flag indicates that this jacobian entry has a resistive component.

#### 6.4.4 JACOBIAN_ENTRY_REACT

```c
#define JACOBIAN_ENTRY_REACT 8
```

This flag indicates that this jacobian entry has a reactive component.

### 6.5 eval argument flags

```c
#define CALC_RESIST_RESIDUAL 1
#define CALC_REACT_RESIDUAL 2
#define CALC_RESIST_JACOBIAN 4
#define CALC_REACT_JACOBIAN 8
#define CALC_NOISE 16
#define CALC_OP 32
#define CALC_RESIST_LIM_RHS 64
#define CALC_REACT_LIM_RHS 128
#define ENABLE_LIM 256
#define INIT_LIM 512
#define ANALYSIS_NOISE 1024
#define ANALYSIS_DC 2048
#define ANALYSIS_AC 4096
#define ANALYSIS_TRAN 8192
#define ANALYSIS_IC 16384
#define ANALYSIS_STATIC 32768
#define ANALYSIS_NODESET 65536
```

The `eval` function allows the simulator to specify which values can be calculated. This can improve performance by avoiding unneeded calculations. To that end the simulator can set bit flags within the `flags` argument of the `eval` function.

#### 6.5.1 CALC_RESIST_RESIDUAL

```c
#define CALC_RESIST_RESIDUAL 1
```

This flag instructs the `eval` function to calculate the resistive residual. The `eval` function must be called with this flag before the `load_residual_resist`, `load_spice_rhs_tran` and `load_spice_rhs_dc` functions are called.

#### 6.5.2 CALC_REACT_RESIDUAL

```c
#define CALC_REACT_RESIDUAL 2
```

This flag instructs the `eval` function to calculate the reactive residual. The `eval` function must be called with this flag before the `load_residual_react` and `load_spice_rhs_tran` functions are called.

#### 6.5.3 CALC_RESIST_JACOBIAN

```c
#define CALC_RESIST_JACOBIAN 4
```

This flag instructs the `eval` function to calculate the resistive jacobian entries. The `eval` function must be called with this flag before the `load_jacobian_resist`, `load_spice_rhs_tran` and `load_spice_rhs_dc` functions are called.

#### 6.5.4 CALC_REACT_JACOBIAN

```c
#define CALC_REACT_JACOBIAN 8
```

This flag instructs the `eval` function to calculate the reactive jacobian entries. The `eval` function must be called with this flag before the `load_jacobian_react`, `load_spice_rhs_tran` and `load_spice_rhs_dc` functions are called.

#### 6.5.5 CALC_NOISE

```c
#define CALC_NOISE 16
```

This flag instructs the `eval` function to calculate the arguments of noise sources. The `eval` function must be called with this flag before the `load_noise` function is called.

#### 6.5.6 CALC_OP

```c
#define CALC_OP 32
```

This flag instructs the `eval` function to calculate operating point variables.

#### 6.5.7 CALC_RESIST_LIM_RHS

```c
#define CALC_RESIST_LIM_RHS 64
```

This flag instructs the `eval` function to calculate the resistive components of the rhs that accounts for `$limit`. The `eval` function must be called with this flag before the `load_limit_rhs_resist` and `load_spice_rhs_dc` functions are called.

#### 6.5.8 CALC_REACT_LIM_RHS

```c
#define CALC_REACT_LIM_RHS 128
```

This flag instructs the `eval` function to calculate the reactive components of the rhs that accounts for `$limit`. The `eval` function must be called with this flag before the `load_limit_rhs_react`, `load_spice_rhs_tran` and `load_spice_rhs_dc` functions are called.

#### 6.5.9 ENABLE_LIM

```c
#define ENABLE_LIM 256
```

This flag instructs the `eval` function to use `$limit` functions. If this flag is not set calls to `$limit` simply return the value of the branch without any limiting.

#### 6.5.10 INIT_LIM

```c
#define INIT_LIM 512
```

This flag instructs the `eval` function to use the initial value for built-in limit functions by passing true to the `init` argument of the callback.

#### 6.5.11 ANALYSIS_NOISE

```c
#define ANALYSIS_NOISE 1024
```

If this flag is set, then `analysis("noise")` returns 1. See section 4.6.1 of Verilog-AMS language reference manual (2.4.0) for details.

#### 6.5.12 ANALYSIS_DC

```c
#define ANALYSIS_DC 2048
```

If this flag is set, then `analysis("dc")` returns 1. See section 4.6.1 of Verilog-AMS language reference manual (2.4.0) for details.

#### 6.5.13 ANALYSIS_AC

```c
#define ANALYSIS_AC 4096
```

If this flag is set, then `analysis("ac")` returns 1. See section 4.6.1 of Verilog-AMS language reference manual (2.4.0) for details.

#### 6.5.14 ANALYSIS_TRAN

```c
#define ANALYSIS_TRAN 8192
```

If this flag is set, then `analysis("tran")` returns 1. See section 4.6.1 of Verilog-AMS language reference manual (2.4.0) for details.

#### 6.5.15 ANALYSIS_IC

```c
#define ANALYSIS_IC 16384
```

If this flag is set, then `analysis("ic")` returns 1. See section 4.6.1 of Verilog-AMS language reference manual (2.4.0) for details. Additionally, while this flag is active the initial condition for `idt` and `idtmod` time integrals is used. Note that initial condition is always used if `CALC_REACT_JACOBIAN` is disabled.

#### 6.5.16 ANALYSIS_STATIC

```c
#define ANALYSIS_STATIC 32768
```

If this flag is set, then `analysis("static")` returns 1. See section 4.6.1 of Verilog-AMS language reference manual (2.4.0) for details.

#### 6.5.17 ANALYSIS_NODESET

```c
#define ANALYSIS_NODESET 65536
```

If this flag is set, then `analysis("nodeset")` returns 1. See section 4.6.1 of Verilog-AMS language reference manual (2.4.0) for details.

### 6.6 eval return flags

```c
#define EVAL_RET_FLAG_LIM 1
#define EVAL_RET_FLAG_FATAL 2
#define EVAL_RET_FLAG_FINISH 4
#define EVAL_RET_FLAG_STOP 8
```

Verilog-A allows behavioral code to control the simulation flow. This functionality is accommodated by setting bit-flags for the return value of the `eval` function.

#### 6.6.1 EVAL_RET_FLAG_LIM

```c
#define EVAL_RET_FLAG_LIM 1
```

This flag indicates that a `$limit` function (like `pnjlim`) has reduced the change of a potential. While this flag is set the simulator must not allow the current simulation to converge.

#### 6.6.2 EVAL_RET_FLAG_FATAL

```c
#define EVAL_RET_FLAG_FATAL 2
```

If this flag is set at a fatal error occurred and the simulator must abort the current simulation with an error.

#### 6.6.3 EVAL_RET_FLAG_FINISH

```c
#define EVAL_RET_FLAG_FINISH 4
```

The `EVAL_RET_FLAG_FINISH` flag indicates that `$finish` was called. If the current iteration has converged the simulator must exit gracefully. Otherwise, this flag should be ignored.

#### 6.6.4 EVAL_RET_FLAG_STOP

```c
#define EVAL_RET_FLAG_STOP 8
```

This flag indicates that `$stop` was called. If the current iteration has converged the simulator must pause the current simulation. Otherwise, this flag should be ignored.

### 6.7 Log Level

```c
#define LOG_LVL_MASK 7
#define LOG_LVL_DEBUG 0
#define LOG_LVL_DISPLAY 1
#define LOG_LVL_INFO 2
#define LOG_LVL_WARN 3
#define LOG_LVL_ERR 4
#define LOG_LVL_FATAL 5
#define LOG_FMT_ERR 16
```

Log message handling is highly simulator specific yet not performance critical. Therefore, OSDI delegates this functionality to the `osdi_log` callback. This callback receives a bitflags that indicates how to handle this message.

#### 6.7.1 LOG_LVL_MASK

```c
#define LOG_LVL_MASK 7
```

The bits masked by this constant contains the log lvl flag. The log lvl indicate what kind of function was used to generate this log message. This level is only intended to determine how, when and if to display the message. The simulation flow should not be changed (e.g. terminated) based upon this flow. The flags documented in section 6.6 are used for that purpose.

**Note (Bug fix from OSDI 0.3):** In OSDI 0.3, this was incorrectly defined as 8. The correct value is 7 (binary `0111`) to mask the lower 3 bits containing log levels 0-5.

#### 6.7.2 LOG_LVL_DEBUG

```c
#define LOG_LVL_DEBUG 0
```

Indicates logging with the `$debug` functions. These messages should be printed immediately.

#### 6.7.3 LOG_LVL_DISPLAY

```c
#define LOG_LVL_DISPLAY 1
```

Indicates logging with the `$strobe` / `$display` / `$write` functions. These messages should be printed after convergence.

#### 6.7.4 LOG_LVL_INFO

```c
#define LOG_LVL_INFO 2
```

Indicates logging with the `$info` function. These messages should be printed after convergence.

#### 6.7.5 LOG_LVL_WARN

```c
#define LOG_LVL_WARN 3
```

Indicates logging with the `$warning` function. These messages should be printed after convergence.

#### 6.7.6 LOG_LVL_ERR

```c
#define LOG_LVL_ERR 4
```

Indicates logging with the `$error` function. These messages should be printed after convergence.

#### 6.7.7 LOG_LVL_FATAL

```c
#define LOG_LVL_FATAL 5
```

Indicates logging with the `$fatal` function. These messages should be printed immediately as the simulation is terminated immediately afterwards.

#### 6.7.8 LOG_FMT_ERR

```c
#define LOG_FMT_ERR 16
```

This flag is set when formatting the raw format string failed. When this flag is set the unprocessed format literal is passed to the `msg` argument. This literal is a constant that must not be freed.

### 6.8 OsdiInitError error-codes

```c
#define INIT_ERR_OUT_OF_BOUNDS 1
```

The `setup_instance` and `setup_model` routines process user inputs (like parameters). These inputs may be invalid, and therefore these routines can produce `OsdiInitError`. The `code` field contains an error code that indicates the error that occurred. These errors are documented below.

#### 6.8.1 INIT_ERR_OUT_OF_BOUNDS

```c
#define INIT_ERR_OUT_OF_BOUNDS 1
```

This error occurs if one of the model or instance parameters violates the boundaries set in the Verilog-A code. In this case the `payload` field contains the `parameter_id` of the parameter with invalid values.

### 6.9 Attribute type constants (New in 0.4)

```c
#define ATTR_TYPE_STR 0
#define ATTR_TYPE_INT 1
#define ATTR_TYPE_REAL 2
```

These constants define the possible types for attribute values in `OsdiAttribute`.

#### 6.9.1 ATTR_TYPE_STR

```c
#define ATTR_TYPE_STR 0
```

Indicates that the attribute value is a string (`char*`).

#### 6.9.2 ATTR_TYPE_INT

```c
#define ATTR_TYPE_INT 1
```

Indicates that the attribute value is an integer (`int32_t`).

#### 6.9.3 ATTR_TYPE_REAL

```c
#define ATTR_TYPE_REAL 2
```

Indicates that the attribute value is a real number (`double`).

### 6.10 Nature reference constants (New in 0.4)

```c
#define NATREF_NONE 0
#define NATREF_NATURE 1
#define NATREF_DISCIPLINE_FLOW 2
#define NATREF_DISCIPLINE_POTENTIAL 3
```

These constants define the possible reference types in `OsdiNatureRef`.

#### 6.10.1 NATREF_NONE

```c
#define NATREF_NONE 0
```

Indicates no nature reference.

#### 6.10.2 NATREF_NATURE

```c
#define NATREF_NATURE 1
```

Indicates a direct reference to a nature. The `index` field of `OsdiNatureRef` contains the index into the nature list.

#### 6.10.3 NATREF_DISCIPLINE_FLOW

```c
#define NATREF_DISCIPLINE_FLOW 2
```

Indicates a reference to the flow nature of a discipline. The `index` field of `OsdiNatureRef` contains the index into the discipline list.

#### 6.10.4 NATREF_DISCIPLINE_POTENTIAL

```c
#define NATREF_DISCIPLINE_POTENTIAL 3
```

Indicates a reference to the potential nature of a discipline. The `index` field of `OsdiNatureRef` contains the index into the discipline list.

### 6.11 Domain constants (New in 0.4)

```c
#define DOMAIN_NOT_GIVEN 0
#define DOMAIN_DISCRETE 1
#define DOMAIN_CONTINUOUS 2
```

These constants define the possible domain types for disciplines.

#### 6.11.1 DOMAIN_NOT_GIVEN

```c
#define DOMAIN_NOT_GIVEN 0
```

Indicates that the domain was not specified.

#### 6.11.2 DOMAIN_DISCRETE

```c
#define DOMAIN_DISCRETE 1
```

Indicates a discrete domain (digital signals).

#### 6.11.3 DOMAIN_CONTINUOUS

```c
#define DOMAIN_CONTINUOUS 2
```

Indicates a continuous domain (analog signals).

---

## 7 Verilog-A Standard Compliance

OSDI is predominantly aimed at compact modelling. In the compact modelling community some parts of the Verilog-A language are de-facto not used. For allowing fast and consistent results, some limitations of the Verilog-A language subset are necessary as defined below.

### 7.1 Hidden State

OSDI compliant compilers must assume that a compiled model does not have hidden states. This often allows more code to be moved into the `model_setup` and `instance_setup` functions, significantly improving performance.

If a variable has the `hidden_state` attribute in Verilog-A, the compiler can not make that assumption. The same attribute can be placed on a Verilog-A module to allow hidden states for all variables within the module.

### 7.2 limexp

The `limexp` function is commonly used in Verilog-A because exponential overflow is a major cause of convergence issues. The language standard defines that `limexp` should limit the change of its argument between iterations. This requires access to values from previous iterations and also a limiting algorithm that is applicable in the general case. As a result all implementations known to the author have opted to use a simple linearized exponential instead, for example:

```c
if (x < EXP_LIM){
   return exp(x)
}else{
   return exp(EXP_LIM) * (x + 1 - EXP_LIM)
}
```

OSDI uses this linearized function as well, with `EXP_LIM = 80`. This may change in future versions if tangible improvements can be demonstrated with a different algorithm.

---

## 8 Files

### 8.1 osdi.h

```c
#pragma once

#ifndef NO_STD
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#endif


#define OSDI_VERSION_MAJOR_CURR 0
#define OSDI_VERSION_MINOR_CURR 4

#define PARA_TY_MASK 3
#define PARA_TY_REAL 0
#define PARA_TY_INT 1
#define PARA_TY_STR 2
#define PARA_KIND_MASK  (3 << 30)
#define PARA_KIND_MODEL (0 << 30)
#define PARA_KIND_INST  (1 << 30)
#define PARA_KIND_OPVAR (2 << 30)

#define ACCESS_FLAG_READ 0
#define ACCESS_FLAG_SET 1
#define ACCESS_FLAG_INSTANCE 4

#define JACOBIAN_ENTRY_RESIST_CONST 1
#define JACOBIAN_ENTRY_REACT_CONST 2
#define JACOBIAN_ENTRY_RESIST 4
#define JACOBIAN_ENTRY_REACT 8

#define CALC_RESIST_RESIDUAL 1
#define CALC_REACT_RESIDUAL 2
#define CALC_RESIST_JACOBIAN 4
#define CALC_REACT_JACOBIAN 8
#define CALC_NOISE 16
#define CALC_OP 32
#define CALC_RESIST_LIM_RHS 64
#define CALC_REACT_LIM_RHS 128
#define ENABLE_LIM 256
#define INIT_LIM 512
#define ANALYSIS_NOISE 1024
#define ANALYSIS_DC 2048
#define ANALYSIS_AC 4096
#define ANALYSIS_TRAN 8192
#define ANALYSIS_IC 16384
#define ANALYSIS_STATIC 32768
#define ANALYSIS_NODESET 65536

#define EVAL_RET_FLAG_LIM 1
#define EVAL_RET_FLAG_FATAL 2
#define EVAL_RET_FLAG_FINISH 4
#define EVAL_RET_FLAG_STOP 8


#define LOG_LVL_MASK 7
#define LOG_LVL_DEBUG 0
#define LOG_LVL_DISPLAY 1
#define LOG_LVL_INFO 2
#define LOG_LVL_WARN 3
#define LOG_LVL_ERR 4
#define LOG_LVL_FATAL 5
#define LOG_FMT_ERR 16

#define INIT_ERR_OUT_OF_BOUNDS 1

#define ATTR_TYPE_STR 0
#define ATTR_TYPE_INT 1
#define ATTR_TYPE_REAL 2

#define NATREF_NONE 0
#define NATREF_NATURE 1
#define NATREF_DISCIPLINE_FLOW 2
#define NATREF_DISCIPLINE_POTENTIAL 3

#define DOMAIN_NOT_GIVEN 0
#define DOMAIN_DISCRETE 1
#define DOMAIN_CONTINUOUS 2


typedef struct OsdiLimFunction {
  char *name;
  uint32_t num_args;
  void *func_ptr;
}OsdiLimFunction;

typedef struct OsdiSimParas {
  char **names;
  double *vals;
  char **names_str;
  char **vals_str;
}OsdiSimParas;

typedef struct OsdiSimInfo {
    OsdiSimParas paras;
    double abstime;
    double *prev_solve;
    double *prev_state;
    double *next_state;
    uint32_t flags;
}OsdiSimInfo;

typedef union OsdiInitErrorPayload {
  uint32_t parameter_id;
}OsdiInitErrorPayload;

typedef struct OsdiInitError {
  uint32_t code;
  OsdiInitErrorPayload payload;
}OsdiInitError;

typedef struct OsdiInitInfo {
  uint32_t flags;
  uint32_t num_errors;
  OsdiInitError *errors;
}OsdiInitInfo;

typedef struct OsdiNodePair {
  uint32_t node_1;
  uint32_t node_2;
}OsdiNodePair;

typedef struct OsdiJacobianEntry {
  OsdiNodePair nodes;
  uint32_t react_ptr_off;
  uint32_t flags;
}OsdiJacobianEntry;

typedef struct OsdiNode {
  char *name;
  char *units;
  char *residual_units;
  uint32_t resist_residual_off;
  uint32_t react_residual_off;
  uint32_t resist_limit_rhs_off;
  uint32_t react_limit_rhs_off;
  bool is_flow;
}OsdiNode;

typedef struct OsdiParamOpvar {
  char **name;
  uint32_t num_alias;
  char *description;
  char *units;
  uint32_t flags;
  uint32_t len;
}OsdiParamOpvar;

typedef struct OsdiNoiseSource {
  char *name;
  OsdiNodePair nodes;
}OsdiNoiseSource;

typedef struct OsdiNatureRef {
  uint32_t ref_type;
  uint32_t index;
}OsdiNatureRef;

typedef struct OsdiDescriptor {
  char *name;

  uint32_t num_nodes;
  uint32_t num_terminals;
  OsdiNode *nodes;

  uint32_t num_jacobian_entries;
  OsdiJacobianEntry *jacobian_entries;

  uint32_t num_collapsible;
  OsdiNodePair *collapsible;
  uint32_t collapsed_offset;

  OsdiNoiseSource *noise_sources;
  uint32_t num_noise_src;

  uint32_t num_params;
  uint32_t num_instance_params;
  uint32_t num_opvars;
  OsdiParamOpvar *param_opvar;

  uint32_t node_mapping_offset;
  uint32_t jacobian_ptr_resist_offset;

  uint32_t num_states;
  uint32_t state_idx_off;

  uint32_t bound_step_offset;

  uint32_t instance_size;
  uint32_t model_size;

  void *(*access)(void *inst, void *model, uint32_t id, uint32_t flags);

  void (*setup_model)(void *handle, void *model, OsdiSimParas *sim_params,
                                     OsdiInitInfo *res);
  void (*setup_instance)(void *handle, void *inst, void *model,
                                     double temperature, uint32_t num_terminals,
                                     OsdiSimParas *sim_params, OsdiInitInfo *res);

  uint32_t (*eval)(void *handle, void *inst, void *model, OsdiSimInfo *info);
  void (*load_noise)(void *inst, void *model, double freq, double *noise_dens);
  void (*load_residual_resist)(void *inst, void* model, double *dst);
  void (*load_residual_react)(void *inst, void* model, double *dst);
  void (*load_limit_rhs_resist)(void *inst, void* model, double *dst);
  void (*load_limit_rhs_react)(void *inst, void* model, double *dst);
  void (*load_spice_rhs_dc)(void *inst, void* model, double *dst,
                  double* prev_solve);
  void (*load_spice_rhs_tran)(void *inst, void* model, double *dst,
                  double* prev_solve, double alpha);
  void (*load_jacobian_resist)(void *inst, void* model);
  void (*load_jacobian_react)(void *inst, void* model, double alpha);
  void (*load_jacobian_tran)(void *inst, void* model, double alpha);
  uint32_t (*given_flag_model)(void *model, uint32_t id);
  uint32_t (*given_flag_instance)(void *inst, uint32_t id);
  uint32_t num_resistive_jacobian_entries;
  uint32_t num_reactive_jacobian_entries;
  void (*write_jacobian_array_resist)(void *inst, void* model, double* destination);
  void (*write_jacobian_array_react)(void *inst, void* model, double* destination);
  uint32_t num_inputs;
  OsdiNodePair* inputs;
  void (*load_jacobian_with_offset_resist)(void *inst, void* model, size_t offset);
  void (*load_jacobian_with_offset_react)(void *inst, void* model, size_t offset);
  OsdiNatureRef* unknown_nature;
  OsdiNatureRef* residual_nature;
}OsdiDescriptor;

typedef struct OsdiNature {
  char *name;
  uint32_t parent_type;
  uint32_t parent;
  uint32_t ddt;
  uint32_t idt;
  uint32_t attr_start;
  uint32_t num_attr;
}OsdiNature;

typedef struct OsdiDiscipline {
  char *name;
  uint32_t flow;
  uint32_t potential;
  uint32_t domain;
  uint32_t attr_start;
  uint32_t num_flow_attr;
  uint32_t num_potential_attr;
  uint32_t num_user_attr;
}OsdiDiscipline;

typedef union OsdiAttributeValue {
  char* string;
  int32_t integer;
  double real;
}OsdiAttributeValue;

typedef struct OsdiAttribute {
  char *name;
  uint32_t value_type;
  OsdiAttributeValue value;
}OsdiAttribute;
```

---

## Changes from OSDI 0.3 to 0.4

### Summary of Changes

1. **Version Update**: `OSDI_VERSION_MINOR_CURR` changed from 3 to 4.

2. **`load_noise` Signature Change**: The `ln_noise_dens` parameter has been removed. In OSDI 0.3, `load_noise` had an additional output parameter `double *ln_noise_dens` that was set to `log(noise_dens[i])` for each noise source. This redundant output has been removed; simulators requiring logarithmic values should compute them from `noise_dens`.

3. **New Routines in OsdiDescriptor**:
   - `given_flag_model` - Query if a model parameter was explicitly set
   - `given_flag_instance` - Query if an instance parameter was explicitly set
   - `write_jacobian_array_resist` - Write resistive jacobian entries to contiguous array
   - `write_jacobian_array_react` - Write reactive jacobian entries to contiguous array
   - `load_jacobian_with_offset_resist` - Load resistive jacobian with pointer offset
   - `load_jacobian_with_offset_react` - Load reactive jacobian with pointer offset

4. **New Fields in OsdiDescriptor**:
   - `num_resistive_jacobian_entries` - Count of jacobian entries with resistive components
   - `num_reactive_jacobian_entries` - Count of jacobian entries with reactive components
   - `num_inputs` - Number of input node pairs
   - `inputs` - Array of input node pairs
   - `unknown_nature` - Nature references for node unknowns
   - `residual_nature` - Nature references for residuals

5. **New Data Structures**:
   - `OsdiNatureRef` - Reference to a nature or discipline nature
   - `OsdiNature` - Describes a Verilog-A nature definition
   - `OsdiDiscipline` - Describes a Verilog-A discipline definition
   - `OsdiAttribute` - Describes a nature/discipline attribute
   - `OsdiAttributeValue` - Union for attribute values

6. **New Constants**:
   - Attribute types: `ATTR_TYPE_STR`, `ATTR_TYPE_INT`, `ATTR_TYPE_REAL`
   - Nature references: `NATREF_NONE`, `NATREF_NATURE`, `NATREF_DISCIPLINE_FLOW`, `NATREF_DISCIPLINE_POTENTIAL`
   - Domain types: `DOMAIN_NOT_GIVEN`, `DOMAIN_DISCRETE`, `DOMAIN_CONTINUOUS`

7. **Bug Fix**: `LOG_LVL_MASK` corrected from 8 to 7. In OSDI 0.3, `LOG_LVL_MASK` was incorrectly defined as 8 (binary `1000`), which would not properly mask log levels 0-5. This has been corrected to 7 (binary `0111`) to properly mask the lower 3 bits containing the log level.

### Migration Notes

Simulators upgrading from OSDI 0.3 to 0.4 should:

1. **Update `load_noise` calls**: Remove the `ln_noise_dens` output parameter. If your simulator used the logarithmic noise density values, compute them from `noise_dens` using `log(noise_dens[i])`.

2. **Update `OsdiDescriptor` handling**: The struct has new fields appended at the end. Ensure your code accounts for the larger struct size and initializes the new function pointers.

3. **Fix `LOG_LVL_MASK` usage**: If you copied the incorrect value of 8 from the OSDI 0.3 header, update it to 7. Code using `lvl & LOG_LVL_MASK` should now work correctly to extract log levels.

4. **Consider new functionality**:
   - Use `given_flag_model`/`given_flag_instance` for efficient parameter querying
   - Use `write_jacobian_array_*` for simulators that prefer contiguous array-based jacobian access
   - Use `load_jacobian_with_offset_*` for simulators that need jacobian loading at different memory offsets
   - Use `unknown_nature`/`residual_nature` for physical unit awareness in multi-physics simulations
