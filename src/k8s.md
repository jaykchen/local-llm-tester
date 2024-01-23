# Chapter 1. From Monolith to Microservices

## Chapter Overview

Most new companies today run their business processes in the cloud. Newer startups and enterprises which realized early enough the direction technology was headed developed their applications for the cloud.

Not all companies were so fortunate. Some built their success decades ago on top of legacy technologies - monolithic applications with all components tightly coupled and almost impossible to separate, a nightmare to manage and deployed on super expensive hardware.

If working for an organization which refers to their main business application "the black box", where nobody knows what happens inside and most logic was never documented, leaving everyone clueless as to what and how things happen from the moment a request enters the application until a response comes out, and you are tasked to convert this business application into a cloud-ready set of applications, then you may be in for a very long and bumpy ride.

## Learning Objectives

By the end of this chapter, you should be able to:

* Explain what a monolith is.
* Discuss the monolith's challenges in the cloud.
* Explain the concept of microservices.
* Discuss microservices advantages in the cloud.
* Describe the transformation path from a monolith to microservices.

## The Legacy Monolith

Although most enterprises believe that the cloud will be the new home for legacy applications, not all legacy applications are a fit for the cloud, at least not yet.

Moving an application to the cloud should be as easy as walking on the beach and collecting pebbles in a bucket and easily carry them wherever needed. A 1000-ton boulder, on the other hand, is not easy to carry at all. This boulder represents the monolith application - sedimented layers of features and redundant logic translated into thousands of lines of code, written in a single, not so modern programming language, based on outdated software architecture patterns and principles.

In time, the new features and improvements added to code complexity, making development more challenging - loading, compiling, and building times increase with every new update. However, there is some ease in administration as the application is running on a single server, ideally a Virtual Machine or a Mainframe.

A monolith has a rather expensive taste in hardware. Being a large, single piece of software which continuously grows, it has to run on a single system which has to satisfy its compute, memory, storage, and networking requirements. The hardware of such capacity is not only complex and extremely pricey, but at times challenging to procure.

Since the entire monolith application runs as a single process, the scaling of individual features of the monolith is almost impossible. It internally supports a hardcoded number of connections and operations. However, scaling the entire application can be achieved by manually deploying a new instance of the monolith on another server, typically behind a load balancing appliance - another pricey solution.

During upgrades, patches or migrations of the monolith application downtime is inevitable and maintenance windows have to be planned well in advance as disruptions in service are expected to impact clients. While there are third party solutions to minimize downtime to customers by setting up monolith applications in a highly available active/passive configuration, they introduce new challenges for system engineers to keep all systems at the same patch level and may introduce new possible licensing costs.

## The Modern Microservice

Pebbles, as opposed to the 1000-ton boulder, are much easier to handle. They are carved out of the monolith, separated from one another, becoming distributed components each described by a set of specific characteristics. Once weighed all together, the pebbles make up the weight of the entire boulder. These pebbles represent loosely coupled microservices, each performing a specific business function. All the functions grouped together form the overall functionality of the original monolithic application. Pebbles are easy to select and group together based on color, size, shape, and require minimal effort to relocate when needed. Try relocating the 1000-ton boulder, effortlessly.

Microservices can be deployed individually on separate servers provisioned with fewer resources - only what is required by each service and the host system itself, helping to lower compute resource expenses.

Microservices-based architecture is aligned with Event-driven Architecture and Service-Oriented Architecture (SOA) principles, where complex applications are composed of small independent processes which communicate with each other through Application Programming Interfaces (APIs) over a network. APIs allow access by other internal services of the same application or external, third-party services and applications.

Each microservice is developed and written in a modern programming language, selected to be the best suitable for the type of service and its business function. This offers a great deal of flexibility when matching microservices with specific hardware when required, allowing deployments on inexpensive commodity hardware.

Although the distributed nature of microservices adds complexity to the architecture, one of the greatest benefits of microservices is scalability. With the overall application becoming modular, each microservice can be scaled individually, either manually or automated through demand-based autoscaling.

Seamless upgrades and patching processes are other benefits of microservices architecture. There is virtually no downtime and no service disruption to clients because upgrades are rolled out seamlessly - one service at a time, rather than having to recompile, rebuild and restart an entire monolithic application. As a result, businesses are able to develop and roll-out new features and updates a lot faster, in an agile approach, having separate teams focusing on separate features, thus being more productive and cost-effective.

## Refactoring

Newer, more modern enterprises possess the knowledge and technology to build cloud-native applications that power their business.

Unfortunately, that is not the case for established enterprises running on legacy monolithic applications. Some have tried to run monoliths as microservices, and as one would expect, it did not work very well. The lessons learned were that a monolithic size multi-process application cannot run as a microservice and that other options had to be explored. The next natural step in the path of the monolith to microservices transition was refactoring. However, migrating a decades-old application to the cloud through refactoring poses serious challenges and the enterprise faces the refactoring approach dilemma: a "Big-bang" approach or an incremental refactoring.

A so-called "Big-bang" approach focuses all efforts with the refactoring of the monolith, postponing the development and implementation of any new features - essentially delaying progress and possibly, in the process, even breaking the core of the business, the monolith.

An incremental refactoring approach guarantees that new features are developed and implemented as modern microservices which are able to communicate with the monolith through APIs, without appending to the monolith's code. In the meantime, features are refactored out of the monolith which slowly fades away while all, or most its functionality is modernized into microservices. This incremental approach offers a gradual transition from a legacy monolith to modern microservices architecture and allows for phased migration of application features into the cloud.

Once an enterprise chooses the refactoring path, there are other considerations in the process. Which business components to separate from the monolith to become distributed microservices, how to decouple the databases from the application to separate data complexity from application logic, and how to test the new microservices and their dependencies, are just a few of the decisions an enterprise is faced with during refactoring.

The refactoring phase slowly transforms the monolith into a cloud-native application which takes full advantage of cloud features, by coding in new programming languages and applying modern architectural patterns. Through refactoring, a legacy monolith application receives a second chance at life - to live on as a modular system adapted to fully integrate with today's fast-paced cloud automation tools and services.

## Challenges

The refactoring path from a monolith to microservices is not smooth and without challenges. Not all monoliths are perfect candidates for refactoring, while some may not even "survive" such a modernization phase. When deciding whether a monolith is a possible candidate for refactoring, there are many possible issues to consider.

When considering a legacy Mainframe based system, written in older programming languages - Cobol or Assembler, it may be more economical to just re-build it from the ground up as a cloud-native application. A poorly designed legacy application should be re-designed and re-built from scratch following modern architectural patterns for microservices and even containers. Applications tightly coupled with data stores are also poor candidates for refactoring.

Once the monolith survived the refactoring phase, the next challenge is to design mechanisms or find suitable tools to keep alive all the decoupled modules to ensure application resiliency as a whole. 

Choosing runtimes may be another challenge. If deploying many modules on a single physical or virtual server, chances are that different libraries and runtime environments may conflict with one another, causing errors and failures. This forces deployments of single modules per servers in order to separate their dependencies - not an economical way of resource management, and no real segregation of libraries and runtimes, as each server also has an underlying Operating System running with its libraries, thus consuming server resources - at times the OS consuming more resources than the application module itself.

Eventually a solution emerged to tackle these refactoring challenges. Application containers came along providing encapsulated lightweight runtime environments for application modules. Containers promised consistent software environments for developers, testers, all the way from Development to Production. Wide support of containers ensured application portability from physical bare-metal to Virtual Machines, but this time with multiple applications deployed on the very same server, each running in their own execution environments isolated from one another, thus avoiding conflicts, errors, and failures. Other features of containerized application environments are higher server utilization, individual module scalability, flexibility, interoperability and easy integration with automation tools.

## Success stories

Although a challenging process, moving from monoliths to microservices is a rewarding journey especially once a business starts to see growth and success delivered by a refactored application system. Below we are listing only a handful of the success stories of companies which rose to the challenge to modernize their monolith business applications. A detailed list of success stories is available at the Kubernetes website: [Kubernetes User Case Studies](https://kubernetes.io/case-studies/).

* AppDirect - an end-to-end commerce platform provider, started from a complex monolith application and through refactoring was able to retain limited functionality monoliths receiving very few commits, but all new features implemented as containerized microservices.
* box - a cloud storage solutions provider, started from a complex monolith architecture and through refactoring was able to decompose it into microservices.
* Crowdfire - a content management solutions provider, successfully broke down their initial monolith into microservices.
* GolfNow - a technology and services provider, decided to break their monoliths apart into containerized microservices.
* Pinterest - a social media services provider, started the refactoring process by first migrating their monolith API.

# Chapter 2. Container Orchestration

## Chapter overview

Container images allow us to confine the application code, its runtime, and all of its dependencies in a pre-defined format. The container runtimes like runC, containerd, or cri-o can use pre-packaged images as a source to create and run one or more containers. These runtimes are capable of running containers on a single host, however, in practice, we would like to have a fault-tolerant and scalable solution, achieved by building a single controller/management unit, a collection of multiple hosts connected together. This controller/management unit is generally referred to as a container orchestrator.

In this chapter, we will explore why we should use container orchestrators, different implementations of container orchestrators, and where to deploy them.

## Learning Objectives

By the end of this chapter, you should be able to:

* Define the concept of container orchestration.
* Explain the benefits of using container orchestration.
* Discuss different container orchestration options.
* Discuss different container orchestration deployment options.

## What Are Containers?

Before we dive into container orchestration, let's review first what containers are.

Containers are an application-centric method to deliver high-performing, scalable applications on any infrastructure of your choice. Containers are best suited to deliver microservices by providing portable, isolated virtual environments for applications to run without interference from other running applications.

Microservices are lightweight applications written in various modern programming languages, with specific dependencies, libraries and environmental requirements. To ensure that an application has everything it needs to run successfully it is packaged together with its dependencies.

Containers encapsulate microservices and their dependencies but do not run them directly. Containers run container images.

A container image bundles the application along with its runtime, libraries, and dependencies, and it represents the source of a container deployed to offer an isolated executable environment for the application. Containers can be deployed from a specific image on many platforms, such as workstations, Virtual Machines, public cloud, etc.

## What Is Container Orchestration?

In Development (Dev) environments, running containers on a single host for development and testing of applications may be a suitable option. However, when migrating to Quality Assurance (QA) and Production (Prod) environments, that is no longer a viable option because the applications and services need to meet specific requirements:

* Fault-tolerance
* On-demand scalability
* Optimal resource usage
* Auto-discovery to automatically discover and communicate with each other
* Accessibility from the outside world
* Seamless updates/rollbacks without any downtime.
* Container orchestrators are tools which group systems together to form clusters where containers' deployment and management is automated at scale while meeting the requirements mentioned above.

## Container Orchestrators

With enterprises containerizing their applications and moving them to the cloud, there is a growing demand for container orchestration solutions. While there are many solutions available, some are mere re-distributions of well-established container orchestration tools, enriched with features and, sometimes, with certain limitations in flexibility.

Although not exhaustive, the list below provides a few different container orchestration tools and services available today:

* Amazon Elastic Container Service (ECS) is a hosted service provided by Amazon Web Services (AWS) to run containers at scale on its infrastructure.
* Azure Container Instance (ACI) is a basic container orchestration service provided by Microsoft Azure.
* Azure Service Fabric is an open source container orchestrator provided by Microsoft Azure.
* Kubernetes is an open source orchestration tool, originally started by Google, today part of the Cloud Native Computing Foundation (CNCF) project.
* Marathon is a framework to run containers at scale on Apache Mesos and DC/OS.
* Nomad is the container and workload orchestrator provided by HashiCorp.
* Docker Swarm is a container orchestrator provided by Docker, Inc. It is part of Docker Engine.

## Why Use Container Orchestrators?

Although we can manually maintain a couple of containers or write scripts to manage the lifecycle of dozens of containers, orchestrators make things much easier for users especially when it comes to managing hundreds and thousands of containers running on a global infrastructure.

Most container orchestrators can:

* Group hosts together while creating a cluster.
* Schedule containers to run on hosts in the cluster based on resources availability.
* Enable containers in a cluster to communicate with each other regardless of the host they are deployed to in the cluster.
* Bind containers and storage resources.
* Group sets of similar containers and bind them to load-balancing constructs to simplify access to containerized applications by creating an interface, a level of abstraction between the containers and the client.
* Manage and optimize resource usage.

Allow for implementation of policies to secure access to applications running inside containers.
With all these configurable yet flexible features, container orchestrators are an obvious choice when it comes to managing containerized applications at scale. In this course, we will explore Kubernetes, one of the most in-demand container orchestration tools available today.

## Where to Deploy Container Orchestrators?

Most container orchestrators can be deployed on the infrastructure of our choice - on bare metal, Virtual Machines, on-premises, on public and hybrid clouds. Kubernetes, for example, can be deployed on a workstation, with or without an isolation layer such as a local hypervisor or container runtime, inside a company's data center, in the cloud on AWS Elastic Compute Cloud (EC2) instances, Google Compute Engine (GCE) VMs, DigitalOcean Droplets, OpenStack, etc.

There are turnkey solutions which allow Kubernetes clusters to be installed, with only a few commands, on top of cloud Infrastructures-as-a-Service, such as GCE, AWS EC2, IBM Cloud, Rancher, VMware Tanzu, and multi-cloud solutions through IBM Cloud Private or StackPointCloud.

Last but not least, there is the managed container orchestration as-a-Service, more specifically the managed Kubernetes as-a-Service solution, offered and hosted by the major cloud providers, such as Amazon Elastic Kubernetes Service (Amazon EKS), Azure Kubernetes Service (AKS), DigitalOcean Kubernetes, Google Kubernetes Engine (GKE), IBM Cloud Kubernetes Service, Oracle Container Engine for Kubernetes, or VMware Tanzu Kubernetes Grid.

# Chapter 3. Kubernetes

## Overview

## Learning Objectives

By the end of this chapter, you should be able to:

* Define Kubernetes.
* Explain the reasons for using Kubernetes.
* Discuss the features of Kubernetes.
* Discuss the evolution of Kubernetes from Borg.
* Explain the role of the Cloud Native Computing Foundation.

## What is Kubernetes

According to the Kubernetes website,

"Kubernetes is an open-source system for automating deployment, scaling, and management of containerized applications".

Kubernetes comes from the Greek word κυβερνήτης, which means helmsman or ship pilot. With this analogy in mind, we can think of Kubernetes as the pilot on a ship of containers.

Kubernetes is also referred to as k8s (pronounced Kate's), as there are 8 characters between k and s.

Kubernetes is highly inspired by the Google Borg system, a container and workload orchestrator for its global operations for more than a decade. It is an open source project written in the Go language and licensed under the Apache License, Version 2.0.

Kubernetes was started by Google and, with its v1.0 release in July 2015, Google donated it to the Cloud Native Computing Foundation (CNCF), one of the largest sub-foundations of the Linux Foundation.

New Kubernetes versions are released in 4 month cycles. The current stable version is 1.26 (as of December 2022).

## From Borg to Kubernetes

According to the abstract of Google's Borg paper, published in 2015,

"Google's Borg system is a cluster manager that runs hundreds of thousands of jobs, from many thousands of different applications, across a number of clusters each with up to tens of thousands of machines".

For more than a decade, Borg has been Google's secret, running its worldwide containerized workloads in production. Services we use from Google, such as Gmail, Drive, Maps, Docs, etc., are all serviced using Borg. 

Among the initial authors of Kubernetes were Google employees who have used Borg and developed it in the past. They poured in their valuable knowledge and experience while designing Kubernetes. Several features/objects of Kubernetes that can be traced back to Borg, or to lessons learned from it, are:

* API servers
* Pods
* IP-per-Pod
* Services
* Labels.

We will explore all of them, and more, in this course.

## Kubernetes features

Kubernetes offers a very rich set of features for container orchestration. Some of its fully supported features are:

* Automatic bin packing
    Kubernetes automatically schedules containers based on resource needs and constraints, to maximize utilization without sacrificing availability.
* Designed for extensibility
    A Kubernetes cluster can be extended with new custom features without modifying the upstream source code.
* Self-healing
    Kubernetes automatically replaces and reschedules containers from failed nodes. It terminates and then restarts containers that become unresponsive to health checks, based on existing rules/policy. It also prevents traffic from being routed to unresponsive containers.
* Horizontal scaling
    With Kubernetes applications are scaled manually or automatically based on CPU or custom metrics utilization.
* Service discovery and load balancing
    Containers receive IP addresses from Kubernetes, while it assigns a single Domain Name System (DNS) name to a set of containers to aid in load-balancing requests across the containers of the set.
Additional fully supported Kubernetes features are:

* Automated rollouts and rollbacks
    Kubernetes seamlessly rolls out and rolls back application updates and configuration changes, constantly monitoring the application's health to prevent any downtime.
* Secret and configuration management
    Kubernetes manages sensitive data and configuration details for an application separately from the container image, in order to avoid a rebuild of the respective image. Secrets consist of sensitive/confidential information passed to the application without revealing the sensitive content to the stack configuration, like on GitHub.
* Storage orchestration
    Kubernetes automatically mounts software-defined storage (SDS) solutions to containers from local storage, external cloud providers, distributed storage, or network storage systems.
* Batch execution
    Kubernetes supports batch execution, long-running jobs, and replaces failed containers.
* IPv4/IPv6 dual-stack
    Kubernetes supports both IPv4 and IPv6 addresses.

There are many additional features currently in alpha or beta phase. They will add great value to any Kubernetes deployment once they become stable features. For example, support for role-based access control (RBAC) is stable only as of the Kubernetes 1.8 release.

## Why Use Kubernetes?

Another one of Kubernetes' strengths is portability. It can be deployed in many environments such as local or remote Virtual Machines, bare metal, or in public/private/hybrid/multi-cloud setups.

Kubernetes extensibility allows it to support and to be supported by many 3rd party open source tools which enhance Kubernetes' capabilities and provide a feature-rich experience to its users. It's architecture is modular and pluggable. Not only does it orchestrate modular, decoupled microservices type applications, but also its architecture follows decoupled microservices patterns. Kubernetes' functionality can be extended by writing custom resources, operators, custom APIs, scheduling rules or plugins.

For a successful open source project, the community is as important as having great code. Kubernetes is supported by a thriving community across the world. It has more than 3,200 contributors, who, over time, have pushed over 111,000 commits. There are meet-up groups in different cities and countries which meet regularly to discuss Kubernetes and its ecosystem. The community is divided into Special Interest Groups (SIGs), groups which focus on special topics, such as scaling, bare metal, networking, storage, etc. We will learn more about them in our last chapter, Kubernetes Community.

## Kubernetes Users

In less than a decade since its debut Kubernetes has become the platform of choice for many enterprises of various sizes to run their workloads. It is a solution for workload management in banking, education, finance and investments, gaming, information technology, media and streaming, online retail, ridesharing, telecommunications, nuclear research, and many other industries. There are numerous user case studies and success stories on the Kubernetes website:

* BlaBlaCar
* BlackRock
* Booking.com
* Box
* CapitalOne
* Haufe Group
* Huawei
* IBM
* ING
* Nokia
* Pearson
* Wikimedia
* And many more.

## Cloud Native Computing Foundation (CNCF)

The [_Cloud Native Computing Foundation_](https://www.cncf.io/) (CNCF) is one of the largest sub-projects hosted by the [_Linux Foundation_](https://www.linuxfoundation.org/). CNCF aims to accelerate the adoption of containers, microservices, and cloud-native applications.
 
CNCF hosts a multitude of projects, with more to be added in the future. CNCF provides resources to each of the projects, but, at the same time, each project continues to operate independently under its pre-existing governance structure and with its existing maintainers. Projects within CNCF are categorized based on their maturity levels: Sandbox, Incubating, and Graduated. At the time of this writing, over a dozen projects had reached Graduated status with many more Incubating and in the Sandbox.
Popular graduated projects:

        * [_Kubernetes_](https://kubernetes.io/) container orchestrator
        * [_etcd_](https://etcd.io/) distributed key-value store
        * [_CoreDNS_](https://coredns.io/) DNS server
        * [_containerd_](http://containerd.io/) container runtime
        * [_Envoy_](https://www.envoyproxy.io/) cloud native proxy
        * [_Fluentd_](http://www.fluentd.org/) for unified logging
        * [_Harbor_](https://goharbor.io/) registry
        * [_Helm_](https://www.helm.sh/) package management for Kubernetes
        * [_Linkerd_](https://linkerd.io/) service mesh for Kubernetes
        * [_Open Policy Agent_](https://www.openpolicyagent.org/) policy engine
        * [_Prometheus_](https://prometheus.io/) monitoring system and time series DB
        * [_Rook_](https://rook.io/) cloud native storage orchestrator for Kubernetes

Key incubating projects:

        * [_argo_](https://argoproj.github.io/) workflow engine for Kubernetes
        * [_Buildpacks.io_](https://buildpacks.io/) builds application images
        * [_CRI-O_](https://cri-o.io/) container runtime
        * [_Contour_](https://projectcontour.io/) ingress controller for Kubernetes
        * [_gRPC_](http://www.grpc.io/) for remote procedure call (RPC)
        * [_CNI_](https://www.cni.dev/) for Linux containers networking
        * [_flux_](https://fluxcd.io/) continuous delivery for Kubernetes
        * [_Knative_](https://knative.dev/) serverless containers in Kubernetes
        * [_KubeVirt_](https://kubevirt.io/) Kubernetes based Virtual Machine manager
        * [_Notary_](https://github.com/theupdateframework/notary) for data security
        * And many [_more_](https://www.cncf.io/projects/).

There are many dynamic projects in the CNCF [_Sandbox_](https://www.cncf.io/sandbox-projects/) geared towards metrics, monitoring, identity, scripting, serverless, nodeless, edge, expecting to achieve Incubating and possibly Graduated status. While many active projects are preparing for takeoff, others are being [_archived_](https://www.cncf.io/archived-projects/) once they become less active and no longer in demand. The first projects to be archived are the [_rkt_](https://github.com/rkt/rkt) container runtime, the distributed [_OpenTracing_](https://www.cncf.io/projects/opentracing/), and [_Brigade_](https://brigade.sh/), an event driven automation tool.
The projects under CNCF cover the entire lifecycle of a cloud-native application, from its execution using container runtimes, to its monitoring and logging. This is very important to meet the goals of CNCF.

## CNCF and Kubernetes

For Kubernetes, the Cloud Native Computing Foundation:

        * Provides a neutral home for the Kubernetes trademark and enforces proper usage.
        * Provides license scanning of core and vendor code.
        * Offers legal guidance on patent and copyright issues.
        * Creates and maintains open source learning [_curriculum_](https://github.com/cncf/curriculum), [_training_](https://www.cncf.io/certification/training/), and certification for Kubernetes and [_cloud native associates_](https://www.cncf.io/certification/kcna/) (KCNA), [_administrators_](https://www.cncf.io/certification/CKA/) (CKA), [_application developers_](https://www.cncf.io/certification/ckad/) (CKAD), and [_security specialists_](https://www.cncf.io/certification/cks/) (CKS).
        * Manages a software conformance [_working group_](https://lists.cncf.io/g/cncf-k8s-conformance).
        * Actively markets Kubernetes.
        * Supports ad hoc activities.
        * Sponsors conferences and meetup events.

# Chapter 4. Kubernetes Architecture

## Chapter Overview

In this chapter, we will explore the Kubernetes architecture, the components of a control plane node, the role of the worker nodes, the cluster state management with etcd and the network setup requirements. We will also learn about the Container Network Interface (CNI), as Kubernetes' network specification.

## Learning Objectives

By the end of this chapter, you should be able to:

* Discuss the Kubernetes architecture.
* Explain the different components of the control plane and worker nodes.
* Discuss cluster state management with etcd.
* Review the Kubernetes network setup requirements.

## Kubernetes Architecture

At a very high level, Kubernetes is a cluster of compute systems categorized by their distinct roles:

* One or more control plane nodes
* One or more worker nodes (optional, but recommended).

## Control Plane Node Overview

The control plane node provides a running environment for the control plane agents responsible for managing the state of a Kubernetes cluster, and it is the brain behind all operations inside the cluster. The control plane components are agents with very distinct roles in the cluster's management. In order to communicate with the Kubernetes cluster, users send requests to the control plane via a Command Line Interface (CLI) tool, a Web User-Interface (Web UI) Dashboard, or an Application Programming Interface (API).

It is important to keep the control plane running at all costs. Losing the control plane may introduce downtime, causing service disruption to clients, with possible loss of business. To ensure the control plane's fault tolerance, control plane node replicas can be added to the cluster, configured in High-Availability (HA) mode. While only one of the control plane nodes is dedicated to actively managing the cluster, the control plane components stay in sync across the control plane node replicas. This type of configuration adds resiliency to the cluster's control plane, should the active control plane node fail.

To persist the Kubernetes cluster's state, all cluster configuration data is saved to a distributed key-value store which only holds cluster state related data, no client workload generated data. The key-value store may be configured on the control plane node (stacked topology), or on its dedicated host (external topology) to help reduce the chances of data store loss by decoupling it from the other control plane agents.

In the stacked key-value store topology, HA control plane node replicas ensure the key-value store's resiliency as well. However, that is not the case with external key-value store topology, where the dedicated key-value store hosts have to be separately replicated for HA, a configuration that introduces the need for additional hardware, hence additional operational costs.

## Control Plane Node Components

A control plane node runs the following essential control plane components and agents:

API Server
Scheduler
Controller Managers
Key-Value Data Store.
In addition, the control plane node runs:

Container Runtime
Node Agent
Proxy
Optional add-ons for cluster-level monitoring and logging.

## Control Plane Node Components: API Server

All the administrative tasks are coordinated by the kube-apiserver, a central control plane component running on the control plane node. The API Server intercepts RESTful calls from users, administrators, developers, operators and external agents, then validates and processes them. During processing the API Server reads the Kubernetes cluster's current state from the key-value store, and after a call's execution, the resulting state of the Kubernetes cluster is saved in the key-value store for persistence. The API Server is the only control plane component to talk to the key-value store, both to read from and to save Kubernetes cluster state information - acting as a middle interface for any other control plane agent inquiring about the cluster's state.

The API Server is highly configurable and customizable. It can scale horizontally, but it also supports the addition of custom secondary API Servers, a configuration that transforms the primary API Server into a proxy to all secondary, custom API Servers, routing all incoming RESTful calls to them based on custom defined rules.

## Control Plane Node Components: Scheduler

The role of the kube-scheduler is to assign new workload objects, such as pods encapsulating containers, to nodes - typically worker nodes. During the scheduling process, decisions are made based on current Kubernetes cluster state and new workload object's requirements. The scheduler obtains from the key-value store, via the API Server, resource usage data for each worker node in the cluster. The scheduler also receives from the API Server the new workload object's requirements which are part of its configuration data. Requirements may include constraints that users and operators set, such as scheduling work on a node labeled with disk==ssd key-value pair. The scheduler also takes into account Quality of Service (QoS) requirements, data locality, affinity, anti-affinity, taints, toleration, cluster topology, etc. Once all the cluster data is available, the scheduling algorithm filters the nodes with predicates to isolate the possible node candidates which then are scored with priorities in order to select the one node that satisfies all the requirements for hosting the new workload. The outcome of the decision process is communicated back to the API Server, which then delegates the workload deployment with other control plane agents.

The scheduler is highly configurable and customizable through scheduling policies, plugins, and profiles. Additional custom schedulers are also supported, then the object's configuration data should include the name of the custom scheduler expected to make the scheduling decision for that particular object; if no such data is included, the default scheduler is selected instead.

A scheduler is extremely important and complex in a multi-node Kubernetes cluster, while in a single-node Kubernetes cluster possibly used for learning and development purposes, the scheduler's job is quite simple.

## Control Plane Node Components: Controller Managers

The controller managers are components of the control plane node running controllers or operator processes to regulate the state of the Kubernetes cluster. Controllers are watch-loop processes continuously running and comparing the cluster's desired state (provided by objects' configuration data) with its current state (obtained from the key-value store via the API Server). In case of a mismatch, corrective action is taken in the cluster until its current state matches the desired state.

The kube-controller-manager runs controllers or operators responsible to act when nodes become unavailable, to ensure container pod counts are as expected, to create endpoints, service accounts, and API access tokens.

The cloud-controller-manager runs controllers or operators responsible to interact with the underlying infrastructure of a cloud provider when nodes become unavailable, to manage storage volumes when provided by a cloud service, and to manage load balancing and routing.

## Control Plane Node Components: Key-Value Data Store

[etcd](https://etcd.io/) is an open source project under the [Cloud Native Computing Foundation (CNCF)](https://www.cncf.io/). etcd is a strongly consistent, distributed key-value data store used to persist a Kubernetes cluster's state. New data is written to the data store only by appending to it, data is never replaced in the data store. Obsolete data is compacted (or shredded) periodically to minimize the size of the data store.

Out of all the control plane components, only the API Server is able to communicate with the etcd data store.

etcd's CLI management tool - `etcdctl`, provides snapshot save and restore capabilities which come in handy especially for a single etcd instance Kubernetes cluster - common in Development and learning environments. However, in Stage and Production environments, it is extremely important to replicate the data stores in HA mode, for cluster configuration data resiliency.

Some Kubernetes cluster bootstrapping tools, such as `kubeadm`, by default, provision stacked etcd control plane nodes, where the data store runs alongside and shares resources with the other control plane components on the same control plane node.

For data store isolation from the control plane components, the bootstrapping process can be configured for an external etcd topology, where the data store is provisioned on a dedicated separate host, thus reducing the chances of an etcd failure.

Both stacked and external etcd topologies support HA configurations. etcd is based on the [Raft Consensus Algorithm](https://web.stanford.edu/~ouster/cgi-bin/papers/raft-atc14) which allows a collection of machines to work as a coherent group that can survive the failures of some of its members. At any given time, one of the nodes in the group will be the leader, and the rest of them will be the followers. etcd gracefully handles leader elections and can tolerate node failure, including leader node failures. Any node can be treated as a leader. 

etcd is written in the Go programming language. In Kubernetes, besides storing the cluster state, etcd is also used to store configuration details such as subnets, ConfigMaps, Secrets, etc.

## Worker Node Overview
A worker node provides a running environment for client applications. These applications are microservices running as application containers. In Kubernetes the application containers are encapsulated in Pods, controlled by the cluster control plane agents running on the control plane node. Pods are scheduled on worker nodes, where they find required compute, memory and storage resources to run, and networking to talk to each other and the outside world. A Pod is the smallest scheduling work unit in Kubernetes. It is a logical collection of one or more containers scheduled together, and the collection can be started, stopped, or rescheduled as a single unit of work. 

Also, in a multi-worker Kubernetes cluster, the network traffic between client users and the containerized applications deployed in Pods is handled directly by the worker nodes, and is not routed through the control plane node.

## Worker Node Components

A worker node has the following components:

* Container Runtime
* Node Agent - kubelet
* Proxy - kube-proxy
* Add-ons for DNS, Dashboard user interface, cluster-level monitoring and logging.

## Worker Node Components: Container Runtime

Although Kubernetes is described as a "container orchestration engine", it lacks the capability to directly handle and run containers. In order to manage a container's lifecycle, Kubernetes requires a **container runtime** on the node where a Pod and its containers are to be scheduled. Runtimes are required on all nodes of a Kubernetes cluster, both control plane and worker. Kubernetes supports several container runtimes:

* [_CRI-O_](https://cri-o.io/)A lightweight container runtime for Kubernetes, supporting [_quay.io_](https://quay.io/) and [_Docker Hub_](https://hub.docker.com/) image registries.
* [_containerd_](https://containerd.io/)A simple, robust, and portable container runtime.
* [_Docker Engine_](https://www.docker.com/)
    A popular and complex container platform which uses **containerd** as a container runtime.
* [_Mirantis Container Runtime_](https://www.mirantis.com/software/container-runtime/)Formerly known as the **Docker Enterprise Edition**.

## Worker Node Components: Node Agent - kubelet
The kubelet is an agent running on each node, control plane and workers, and communicates with the control plane. It receives Pod definitions, primarily from the API Server, and interacts with the container runtime on the node to run containers associated with the Pod. It also monitors the health and resources of Pods running containers.

The kubelet connects to container runtimes through a plugin based interface - the [Container Runtime Interface](https://github.com/kubernetes/community/blob/master/contributors/devel/sig-node/container-runtime-interface.md) (CRI). The CRI consists of protocol buffers, gRPC API, libraries, and additional specifications and tools. In order to connect to interchangeable container runtimes, kubelet uses a CRI shim, an application which provides a clear abstraction layer between kubelet and the container runtime. 

As shown above, the kubelet acting as grpc client connects to the CRI shim acting as grpc server to perform container and image operations. The CRI implements two services: ImageService and RuntimeService. The ImageService is responsible for all the image-related operations, while the RuntimeService is responsible for all the Pod and container-related operations.

## Worker Node Components: kubelet - CRI shims

Originally the kubelet agent supported only a couple of container runtimes, first the Docker Engine followed by rkt, through a unique interface model integrated directly in the kubelet source code. However, this approach was not intended to last forever even though it was especially beneficial for Docker. In time, Kubernetes started migrating towards a standardized approach to container runtime integration by introducing the CRI. Kubernetes adopted a decoupled and flexible method to integrate with various container runtimes without the need to recompile its source code. Any container runtime that implements the CRI could be used by Kubernetes to manage containers.

Shims are Container Runtime Interface (CRI) implementations, interfaces or adapters, specific to each container runtime supported by Kubernetes. Below we present some examples of CRI shims:

**cri-containerd**
cri-containerd allows containers to be directly created and managed with containerd at kubelet's request:
 
**CRI-O**
CRI-O enables the use of any Open Container Initiative (OCI) compatible runtime with Kubernetes, such as runC:

 
**dockershim** and **cri-dockerd**
Before Kubernetes release v1.24 the **dockershim** allowed containers to be created and managed by invoking the Docker Engine and its internal runtime containerd. Due to Docker Engine's popularity, this shim has been the default interface used by kubelet. However, starting with Kubernetes release v1.24, the dockershim is no longer being maintained by the Kubernetes project, its specific code is removed from kubelet source code, thus will no longer be supported by the kubelet node agent of Kubernetes. As a result, [_Docker, Inc._,](https://www.docker.com/)and [_Mirantis_](https://www.mirantis.com/) have agreed to introduce and maintain a replacement adapter, **cri-dockerd** that would ensure that the Docker Engine will continue to be a container runtime option for Kubernetes, in addition to the Mirantis Container Runtime (MCR). The introduction of cri-dockerd also ensures that both Docker Engine and MCR follow the same standardized integration method as the CRI-compatible runtimes.
 
 
Additional details about the deprecation process of the dockershim can be found on the [_Updated: Dockershim Removal FAQ_](https://kubernetes.io/blog/2022/02/17/dockershim-faq/) page.

## Worker Node Components: Proxy - kube-proxy

The kube-proxy is the network agent which runs on each node, control plane and workers, responsible for dynamic updates and maintenance of all networking rules on the node. It abstracts the details of Pods networking and forwards connection requests to the containers in the Pods. 

The kube-proxy is responsible for TCP, UDP, and SCTP stream forwarding or random forwarding across a set of Pod backends of an application, and it implements forwarding rules defined by users through Service API objects.

## Worker Node Components: Add-ons

**Add-ons** are cluster features and functionality not yet available in Kubernetes, therefore implemented through 3rd-party pods and services.

* **DNS
    **Cluster DNS is a DNS server required to assign DNS records to Kubernetes objects and resources.
* **Dashboard** 
    A general purpose web-based user interface for cluster management.
* **Monitoring** 
    Collects cluster-level container metrics and saves them to a central data store.
* **Logging** 
    Collects cluster-level container logs and saves them to a central log store for analysis.

## Networking Challenges

Decoupled microservices based applications rely heavily on networking in order to mimic the tight-coupling once available in the monolithic era. Networking, in general, is not the easiest to understand and implement. Kubernetes is no exception - as a containerized microservices orchestrator it needs to address a few distinct networking challenges:

* Container-to-Container communication inside Pods
* Pod-to-Pod communication on the same node and across cluster nodes
* Service-to-Pod communication within the same namespace and across cluster namespaces
* External-to-Service communication for clients to access applications in a cluster

All these networking challenges must be addressed before deploying a Kubernetes cluster.

## Container-to-Container Communication Inside Pods

Making use of the underlying host operating system's kernel virtualization features, a container runtime creates an isolated network space for each container it starts. On Linux, this isolated network space is referred to as a network namespace. A network namespace can be shared across containers, or with the host operating system.

When a grouping of containers defined by a Pod is started, a special infrastructure Pause container is initialized by the Container Runtime for the sole purpose of creating a network namespace for the Pod. All additional containers, created through user requests, running inside the Pod will share the Pause container's network namespace so that they can all talk to each other via localhost.

## Pod-to-Pod Communication Across Nodes

In a Kubernetes cluster Pods, groups of containers, are scheduled on nodes in a nearly unpredictable fashion. Regardless of their host node, Pods are expected to be able to communicate with all other Pods in the cluster, all this without the implementation of Network Address Translation (NAT). This is a fundamental requirement of any networking implementation in Kubernetes.

The Kubernetes network model aims to reduce complexity, and it treats Pods as VMs on a network, where each VM is equipped with a network interface - thus each Pod receiving a unique IP address. This model is called "**IP-per-Pod**" and ensures Pod-to-Pod communication, just as VMs are able to communicate with each other on the same network.

Let's not forget about containers though. They share the Pod's network namespace and must coordinate ports assignment inside the Pod just as applications would on a VM, all while being able to communicate with each other on **localhost** - inside the Pod. However, containers are integrated with the overall Kubernetes networking model through the use of the [_Container Network Interface_](https://github.com/containernetworking/cni) (CNI) supported by [_CNI plugins_](https://github.com/containernetworking/cni#3rd-party-plugins). CNI is a set of specifications and libraries which allow plugins to configure the networking for containers. While there are a few [_core plugins_](https://github.com/containernetworking/plugins#plugins), most CNI plugins are 3rd-party Software Defined Networking (SDN) solutions implementing the Kubernetes networking model. In addition to addressing the fundamental requirement of the networking model, some networking solutions offer support for Network Policies. [_Flannel_](https://github.com/coreos/flannel/), [_Weave_](https://www.weave.works/oss/net/), [_Calico_](https://www.tigera.io/project-calico/) are only a few of the SDN solutions available for Kubernetes clusters.
 
The container runtime offloads the IP assignment to CNI, which connects to the underlying configured plugin, such as Bridge or MACvlan, to get the IP address. Once the IP address is given by the respective plugin, CNI forwards it back to the requested container runtime.
 
For more details, you can explore the [_Kubernetes documentation_](https://kubernetes.io/docs/concepts/cluster-administration/networking/).

## External-to-Pod Communication

A successfully deployed containerized application running in Pods inside a Kubernetes cluster may require accessibility from the outside world. Kubernetes enables external accessibility through Services, complex encapsulations of network routing rule definitions stored in iptables on cluster nodes and implemented by kube-proxy agents. By exposing services to the external world with the aid of kube-proxy, applications become accessible from outside the cluster over a virtual IP address and a dedicated port number.

# Chapter 5. Installing Kubernetes

## Chapter Overview

In this chapter, we will explore Kubernetes cluster deployment considerations. First, we will learn about Kubernetes cluster configuration options, followed by infrastructure requirements and installation tools specific to various cluster deployment models.

## Learning Objectives

By the end of this chapter, you should be able to:

* Discuss Kubernetes configuration options.
* Discuss infrastructure considerations before installing Kubernetes.
* Discuss infrastructure choices for a Kubernetes cluster deployment.
* Review Kubernetes installation tools and certified solutions.

## Kubernetes Configuration

Kubernetes can be installed using different cluster configurations. The major installation types are described below:

* **All-in-One Single-Node Installation**
    In this setup, all the control plane and worker components are installed and running on a single-node. While it is useful for learning, development, and testing, it is not recommended for production purposes.
* **Single-Control Plane and Multi-Worker Installation**
    In this setup, we have a single-control plane node running a stacked etcd instance. Multiple worker nodes can be managed by the control plane node.
* **Single-Control Plane with Single-Node etcd, and Multi-Worker Installation**
    In this setup, we have a single-control plane node with an external etcd instance. Multiple worker nodes can be managed by the control plane node.
* **Multi-Control Plane and Multi-Worker Installation**
    In this setup, we have multiple control plane nodes configured for High-Availability (HA), with each control plane node running a stacked etcd instance. The etcd instances are also configured in an HA etcd cluster and multiple worker nodes can be managed by the HA control plane.
* **Multi-Control Plane with Multi-Node etcd, and Multi-Worker Installation**
    In this setup, we have multiple control plane nodes configured in HA mode, with each control plane node paired with an external etcd instance. The external etcd instances are also configured in an HA etcd cluster, and multiple worker nodes can be managed by the HA control plane. This is the most advanced cluster configuration recommended for production environments. 

As the Kubernetes cluster's complexity grows, so does its hardware and resources requirements. While we can deploy Kubernetes on a single host for learning, development, and possibly testing purposes, the community recommends multi-host environments that support High-Availability control plane setups and multiple worker nodes for client workload for production purposes. 

## Infrastructure for Kubernetes Installation

Once we decide on the installation type, we need to decide on the infrastructure. Infrastructure related decisions are typically guided by the desired environment type, either learning or production environment. For infrastructure, we need to decide on the following:

* Should we set up Kubernetes on bare metal, public cloud, private, or hybrid cloud?
* Which underlying OS should we use? Should we choose a Linux distribution - Red Hat-based or Debian-based, or Windows?
* Which networking solution (CNI) should we use?

Explore the [Kubernetes documentation](https://kubernetes.io/docs/setup/) for details on choosing the right solution.

## Installing Local Learning Clusters

There are a variety of installation tools allowing us to deploy single- or multi-node Kubernetes clusters on our workstations, for learning and development purposes. While not an exhaustive list, below we enumerate a few popular ones:

* [Minikube](https://minikube.sigs.k8s.io/docs/) 
    Single- and multi-node local Kubernetes cluster, recommended for a learning environment deployed on a single host.
* [Kind](https://kind.sigs.k8s.io/) 
    Multi-node Kubernetes cluster deployed in Docker containers acting as Kubernetes nodes, recommended for a learning environment.
* [Docker Desktop](https://www.docker.com/products/docker-desktop) 
    Including a local Kubernetes cluster for Docker users. 
* [MicroK8s](https://microk8s.io/) 
    Local and cloud Kubernetes cluster for developers and production, from Canonical.
* [K3S](https://k3s.io/) 
    Lightweight Kubernetes cluster for local, cloud, edge, IoT deployments, originally from Rancher, currently a CNCF project.

Minikube is an easy and flexible method to create a local Kubernetes setup. We will be using it extensively in this course to manage certain aspects of a Kubernetes cluster, while taking advantage of several automated features designed to simplify the user interaction with the Kubernetes environment and the containerized applications deployed to the cluster.

## Installing Production Clusters with Deployment Tools

When it comes to production ready solutions, there are several recommended tools for Kubernetes clusters bootstrapping and a few that are also capable of provisioning the necessary hosts on the underlying infrastructure.
Let's take a look at the most popular installation tools available:

**kubeadm
**[kubeadm](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/) is a first-class citizen of the Kubernetes ecosystem. It is a secure and recommended method to bootstrap a multi-node production ready Highly Available Kubernetes cluster, on-premises or in the cloud. kubeadm can also bootstrap a single-node cluster for learning. It has a set of building blocks to set up the cluster, but it is easily extendable to add more features. Please note that kubeadm does not support the provisioning of hosts - they should be provisioned separately with a tool of our choice.
 
**kubespray**
[kubespray](https://kubernetes.io/docs/setup/production-environment/tools/kubespray/) (formerly known as kargo) allows us to install Highly Available production ready Kubernetes clusters on AWS, GCP, Azure, OpenStack, vSphere, or bare metal. kubespray is based on Ansible, and is available on most Linux distributions. It is a [Kubernetes Incubator](https://github.com/kubernetes-sigs/kubespray/) project.

**kops**
[kops](https://kubernetes.io/docs/setup/production-environment/tools/kops/) enables us to create, upgrade, and maintain production-grade, Highly Available Kubernetes clusters from the command line. It can provision the required infrastructure as well. Currently, AWS is officially supported. Support for DigitalOcean and OpenStack is in beta, Azure and GCE is in alpha support, and other platforms are planned for the future. Explore the [kops project](https://github.com/kubernetes/kops/) for more details.
 
In addition, for a manual installation approach, the [*Kubernetes The Hard Way*](https://github.com/kelseyhightower/kubernetes-the-hard-way) GitHub project by [Kelsey Hightower](https://twitter.com/kelseyhightower) is an extremely helpful installation guide and resource. The project aims to teach all the detailed steps involved in the bootstrapping of a Kubernetes cluster, steps that are otherwise automated by various tools mentioned in this chapter and obscured from the end user.

## Production Clusters from Certified Solutions Providers

The growing popularity of Kubernetes accelerated its adoption by many cloud services providers together with hosted platforms of certified Kubernetes distributions. There are well over 200 managed [_certified Kubernetes_](https://kubernetes.io/partners/) services providers today, as many more organizations became Kubernetes partners, joining the list of initial providers of hosted Kubernetes solutions:

**Hosted Solutions**
Hosted Solutions providers fully manage the provided software stack, while the user pays hosting and management charges. Popular vendors providing hosted solutions for Kubernetes are (listed in alphabetical order):

* [_Alibaba Cloud Container Service for Kubernetes_](https://www.alibabacloud.com/product/kubernetes) (ACK)
* [_Amazon Elastic Kubernetes Service_](https://aws.amazon.com/eks/) (EKS)
* [_Azure Kubernetes Service_](https://azure.microsoft.com/en-us/services/kubernetes-service/) (AKS)
* [_DigitalOcean Kubernetes_](https://www.digitalocean.com/products/kubernetes/)
* [_Google Kubernetes Engine_](https://cloud.google.com/kubernetes-engine/) (GKE)
* [_IBM Cloud Kubernetes Service_](https://www.ibm.com/cloud/kubernetes-service/)
* [_Oracle Container Engine for Kubernetes_](https://www.oracle.com/cloud-native/container-engine-kubernetes/) (OKE)
* [_Platform9 Managed Kubernetes_](https://platform9.com/managed-kubernetes/) (PMK)
* [_Red Hat OpenShift_](https://www.redhat.com/en/technologies/cloud-computing/openshift)
* [_VMware Tanzu Kubernetes Grid_](https://tanzu.vmware.com/kubernetes-grid)

**Partners**
Additional [_Partners_](https://kubernetes.io/partners/) providing managed Kubernetes services and platforms (listed in alphabetical order):

* Altoros
* Aqua Security
* Canonical
* D2IQ
* Dell Technologies Consulting
* Deloitte
* Fujitsu
* GitLab
* HPE
* Inovex
* Kubermatic
* Kublr
* Mirantis
* Nirmata
* Rancher
* SAP
* Sysdig
* Weaveworks

**Turnkey Cloud Solutions**
[_Turnkey Cloud Solutions_](https://kubernetes.io/docs/setup/production-environment/turnkey-solutions/) install production ready Kubernetes clusters on cloud infrastructure:

* Linode Kubernetes Engine
* Nirmata Managed Kubernetes
* Nutanix Karbon
* Vultr Kubernetes Engine

## Kubernetes on Windows

The Windows operating system plays a key role in running and managing enterprise applications and services. With that in mind, the Kubernetes community worked very hard to bring Windows support to Kubernetes.

With the release of Kubernetes v1.14, Windows was successfully introduced as a [supported](https://kubernetes.io/docs/setup/production-environment/windows/intro-windows-in-kubernetes/) production ready operating system only for worker nodes of a Kubernetes cluster. This enabled Kubernetes to support the deployment of Windows containers in the cluster, either as a dedicated Windows cluster, or a hybrid cluster with Windows nodes running alongside Linux nodes. Keep in mind, however, that the control plane nodes are limited to running on Linux only, with no plans to extend the support to Windows control plane nodes.

With Windows Server 2019 being the only Windows OS supported by Kubernetes, the same container workload orchestration tool can [schedule](https://kubernetes.io/docs/setup/production-environment/windows/user-guide-windows-containers/) and deploy both Linux and Windows containers in the same cluster. The user is responsible to configure the workload scheduling according to the expected OS, that is to schedule Linux and Windows containers on nodes with their respective operating systems when nodes of each OS are found in the same Kubernetes cluster.

# Chapter 6. Minikube - Installing Local Kubernetes Clusters

## Chapter Overview

[Minikube](https://minikube.sigs.k8s.io/) is one of the easiest, most flexible and popular methods to run an all-in-one or a multi-node local Kubernetes cluster, isolated by Virtual Machines (VM) or Containers, run directly on our workstations. Minikube is the tool responsible for the installation of Kubernetes components, cluster bootstrapping, and cluster tear-down when no longer needed. It includes additional features aimed to ease the user interaction with the Kubernetes cluster, but nonetheless, it initializes for us a fully functional, non-production, Kubernetes cluster extremely convenient for learning purposes. Minikube can be installed on native macOS, Windows, and many Linux distributions.

In this chapter, we will explore the requirements to install Minikube locally on our workstation.

## Learning Objectives

By the end of this chapter, you should be able to:

* Understand Minikube.
* Install Minikube on the native OS of your workstation.
* Verify the local installation.

## What Is Minikube?

Minikube is one of the easiest, most flexible and popular methods to run an all-in-one or a multi-node local Kubernetes cluster directly on our local workstations. It installs and runs on any native OS such as Linux, macOS, or Windows. However, in order to fully take advantage of all the features Minikube has to offer, a [_Type-2 Hypervisor_](https://en.wikipedia.org/wiki/Hypervisor) or a Container Runtime should be installed on the local workstation, to run in conjunction with Minikube. The role of the hypervisor or container runtime is to offer an isolated infrastructure for the Minikube Kubernetes cluster components, that is easily reproducible, easy to use and tear down. This isolation of the cluster components from our daily environment ensures that once no longer needed, the Minikube components can be safely removed leaving behind no configuration changes to our workstation, thus no traces of their existence. This does not mean, however, that we are responsible for the provisioning of any VMs or containers with guest operating systems with the help of the hypervisor or container runtime. Minikube includes the necessary adapters to interact directly with the isolation software of choice to build all its infrastructure as long as the Type-2 Hypervisor or Container Runtime is installed on our workstation.

Minikube is built on the capabilities of the [libmachine](https://github.com/docker/machine/tree/master/libmachine) library originally designed by Docker to build [_Virtual Machine container hosts_](https://github.com/docker/machine) on any physical infrastructure. In time Minikube became very flexible, supporting several hypervisors and container runtimes, depending on the host workstation's native OS.

For those who feel more adventurous, Minikube can be installed without an isolation software, on bare-metal, which may result in permanent configuration changes to the host OS. To prevent such permanent configuration changes, a second form of isolation can be achieved by installing Minikube inside a Virtual Machine provisioned with a Type-2 Hypervisor of choice, and a desktop guest OS of choice (with enabled GUI). As a result, when installed inside a VM, Minikube will end up making configuration changes to the guest environment, still isolated from the host workstation. Therefore, now we have two distinct methods to isolate the Minikube environment from our host workstation.

The isolation software can be specified by the user with the **`--driver`** option, otherwise Minikube will try to find a preferred method for the host OS of the workstation.

Once decided on the isolation method, the next step is to determine the required number of Kubernetes cluster nodes, and their sizes in terms of CPU, memory, and disk space. Minikube invokes the hypervisor of choice to provision the infrastructure VM(s) which will host the Kubernetes cluster node(s), or the runtime of choice to run infrastructure container(s) that host the cluster node(s). Keep in mind that Minikube now supports all-in-one single-node and multi-node clusters. Regardless of the isolation method and the expected cluster and node sizes, a local Minikube Kubernetes cluster will ultimately be impacted and/or limited by the physical resources of the host workstation. We have to be mindful of the needs of the host OS and any utilities it may be running, then the needs of the hypervisor or the container runtime, and finally the remaining resources that can be allocated to our Kubernetes cluster. For a learning environment the recommendations are that a Kubernetes node has 2 CPU cores (or virtual CPUs) at a minimum, at least 2 GB of RAM memory (with 4 - 8 GB of RAM recommended for optimal usage), and 20+ GB of disk storage space. When migrating towards a larger, more dynamic, production grade cluster, these resource values should be adjusted accordingly. The Kubernetes nodes are expected to access the internet as well, for software updates, container image downloads, and for client accessibility.

Following the node(s)' provisioning phase, Minikube invokes [kubeadm](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/), to bootstrap the Kubernetes cluster components inside the previously provisioned node(s). We need to ensure that we have the necessary hardware and software required by Minikube to build our environment.

## Requirements for Running Minikube

Below we outline the requirements to run Minikube on our local workstation:

* VT-x/AMD-v virtualization must be enabled on the local workstation, and/or verify if it is supported.
* [_kubectl_](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
    **`kubectl`** is a binary used to access and manage any Kubernetes cluster. It is installed through Minikube and accessed through the **`minikube kubectl`** command, or it can be installed separately and run as a standalone tool. We will explore **`kubectl`** installation and usage in future chapters.
* Type-2 Hypervisor or Container Runtime
    Without a specified driver, Minikube will try to find an installed hypervisor or a runtime, in the following order of preference (on a Linux host): docker, kvm2, podman, vmware, and virtualbox. If multiple isolation software installations are found, such as docker and virtualbox, Minikube will pick docker over virtualbox if no desired driver is specified by the user. Hypervisors and Container Runtimes supported by various native workstation OSes:
    - On Linux [_VirtualBox_](https://www.virtualbox.org/wiki/Downloads), [_KVM2_](https://www.linux-kvm.org/page/Main_Page), and [_QEMU_](https://www.qemu.org/) hypervisors, or [_Docker_](https://docs.docker.com/engine/install/) and [_Podman_](https://podman.io/getting-started/installation.html) runtimes
    - On macOS [_VirtualBox_](https://www.virtualbox.org/wiki/Downloads), [_HyperKit_](https://github.com/moby/hyperkit), [_VMware Fusion_](http://www.vmware.com/products/fusion.html), [_Parallels_](https://www.parallels.com/), and [_QEMU_](https://www.qemu.org/) hypervisors, or [_Docker_](https://docs.docker.com/desktop/mac/install/) runtime
    - On Windows [_VirtualBox_](https://www.virtualbox.org/wiki/Downloads), [_Hyper-V_](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/quick-start/enable-hyper-v), [_VMware Workstation_](https://www.vmware.com/in/products/workstation-pro/workstation-pro-evaluation.html), and [_QEMU_](https://www.qemu.org/) hypervisors, or [_Docker_](https://docs.docker.com/desktop/windows/install/) runtime.

***NOTE:**** Minikube supports a *[***_`--driver=none`_***](https://minikube.sigs.k8s.io/docs/drivers/none/)* (on Linux) option that runs the Kubernetes components bare-metal, directly on the host OS and not inside a VM. With this option a Docker installation is required and a Linux OS on the local workstation, but no hypervisor installation. This driver is recommended for advanced users.*

* Internet connection on the first Minikube run - to download packages, dependencies, updates and pull images needed to initialize the Minikube Kubernetes cluster components. Subsequent Minikube runs will require an Internet connection only when new container images need to be pulled from a public container registry or when deployed containerized applications need it for client accessibility. Once a container image has been pulled, it can be reused from the local container runtime image cache without an Internet connection.

In this chapter, we use one of the most robust and stable isolation methods as a driver, the VirtualBox hypervisor, to provision the VM(s) which host the components of the Kubernetes cluster. While no longer the preferred driver due to slower startup times when compared with other methods, it is still one of the most stable drivers for Minikube.
Read more about Minikube from the official [_Minikube documentation_](https://minikube.sigs.k8s.io/docs/), the official [_Kubernetes documentation_](https://kubernetes.io/docs/tasks/tools/#minikube), or [_GitHub_](https://github.com/kubernetes/minikube).

## Installing Minikube on Linux

Let's learn how to install the latest Minikube release on Ubuntu Linux 20.04 LTS with VirtualBox v7.0 specifically. This installation assumes no other isolation software is installed on our Linux workstation, such as KVM2, QEMU, Docker Engine or Podman, that Minikube can use as a driver.

***NOTE****: For other Linux OS distributions or releases, VirtualBox and Minikube versions, the installation steps may vary! Check the *[*_Minikube installation_*](https://kubernetes.io/docs/tasks/tools/install-minikube/)*!*

Verify the virtualization support on your Linux OS in a terminal (a non-empty output indicates supported virtualization):
**`$ grep -E --color 'vmx|svm' /proc/cpuinfo`**
The easiest way to download and install the [_VirtualBox_](https://www.virtualbox.org/wiki/Linux_Downloads) hypervisor for Linux is from its official download site. However, if feeling adventurous, in a terminal run the following commands to add the recommended source repository for the host OS, download and register the public key, update and install:


```
$ sudo bash -c 'echo "deb \
 [arch=amd64 signed-by=/usr/share/keyrings/oracle-virtualbox-2016.gpg] \
 https://download.virtualbox.org/virtualbox/debian \
 eoan contrib" >> /etc/apt/sources.list'
$ wget -O- \
 https://www.virtualbox.org/download/oracle_vbox_2016.asc | \
 sudo gpg --dearmor --yes \
 --output /usr/share/keyrings/oracle-virtualbox-2016.gpg
$ sudo apt update
$ sudo apt install -y virtualbox-7.0
```


Minikube can be downloaded and installed, in a terminal, the latest release or a specific release from the [_Minikube release page_](https://github.com/kubernetes/minikube/releases):


```
$ curl -LO \
 https://storage.googleapis.com/minikube/releases/latest/minikube_latest_amd64.deb
$ sudo dpkg -i minikube_latest_amd64.deb
```


***NOTE****: Replacing ****`/latest/`**** with a particular version, such as ****`/v1.27.1/`**** will download that specified Minikube version.*
Start Minikube. In a terminal we can start Minikube with the **`minikube start`** command, which bootstraps a single-node cluster with the latest stable Kubernetes version release. For a specific Kubernetes version the **`--kubernetes-version`** option can be used as such **`minikube start --kubernetes-version v1.25.1`** (where **`latest`** is default and acceptable version value, and **`stable`** is also acceptable). More advanced start options will be explored later in this chapter:


```
$ minikube start
😄 minikube v1.28.0 on Ubuntu 20.04
✨ Automatically selected the virtualbox driver. Other choices: none, ssh
💿 Downloading VM boot image ...
 > minikube-v1.28.0-amd64.iso....: 65 B / 65 B [----------] 100.00% ? p/s 0s
 > minikube-v1.28.0-amd64.iso: 274.45 MiB / 274.45 MiB 100.00% 32.75 MiB p/
👍 Starting control plane node minikube in cluster minikube
💾 Downloading Kubernetes v1.25.3 preload ...
 > preloaded-images-k8s-v18-v1...: 385.44 MiB / 385.44 MiB 100.00% 38.52 MiB
🔥 Creating virtualbox VM (CPUs=2, Memory=6000MB, Disk=20000MB) ...
🐳 Preparing Kubernetes v1.25.3 on Docker 20.10.20 ...
 ▪ Generating certificates and keys ...
 ▪ Booting up control plane ...
 ▪ Configuring RBAC rules ...
🔎 Verifying Kubernetes components...
 ▪ Using image gcr.io/k8s-minikube/storage-provisioner:v5
🌟 Enabled addons: default-storageclass, storage-provisioner
💡 kubectl not found. If you need it, try: 'minikube kubectl -- get pods -A'
🏄 Done! kubectl is now configured to use "minikube" cluster and "default" namespace by default
```


***NOTE****: An error message that reads "Unable to pick a default driver..." means that Minikube was not able to locate any one of the supported hypervisors or runtimes. The recommendation is to install or re-install a desired isolation tool, and ensure its executable is found in the default ****`PATH`**** of your OS distribution.*
Check the status. With the **`minikube status`** command, we display the status of the Minikube cluster:

```
$ minikube status
minikube
type: Control Plane
host: Running
kubelet: Running
apiserver: Running
kubeconfig: Configured
```

Stop Minikube. With the **`minikube stop`** command, we can stop Minikube:

```
$ minikube stop
✋ Stopping node "minikube" ...
🛑 1 node stopped.
```

## Installing Minikube on macOS

Let's learn how to install the latest Minikube release on macOS with VirtualBox v7.0 specifically. This installation assumes no other isolation software is installed on our Mac workstation, such as HyperKit, VMware Fusion, Parallels, QEMU or Docker Engine, that Minikube can use as a driver.

***NOTE****: For other VirtualBox and Minikube versions the installation steps may vary! Check the *[*_Minikube installation_*](https://kubernetes.io/docs/tasks/tools/install-minikube/)*!*

Verify the virtualization support on your macOS in a terminal (VMX in the output indicates enabled virtualization):

```
$ sysctl -a | grep -E --color 'machdep.cpu.features|VMX'
```

Install the [_VirtualBox_](https://www.virtualbox.org/wiki/Downloads) hypervisor for 'OS X hosts'. Download and install the **`.dmg`** package.
Install Minikube. We can download and install in a terminal the latest release or a specific release from the [_Minikube release page_](https://github.com/kubernetes/minikube/releases):


```
$ curl -LO \
 https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-amd64
$ sudo install minikube-darwin-amd64 /usr/local/bin/minikube
```

***NOTE****: Replacing ****`/latest/`**** with a particular version, such as ****`/v1.27.1/`**** will download that specified version.*
Start Minikube. We can start Minikube with the **`minikube start`** command, which bootstraps a single-node cluster with the latest stable Kubernetes version release. For a specific Kubernetes version the **`--kubernetes-version`** option can be used as such **`minikube start --kubernetes-version v1.25.1`** (where **`latest`** is default and acceptable version value, and **`stable`** is also acceptable). More advanced start options will be explored later in this chapter:

```
$ minikube start
😄 minikube v1.28.0 on Darwin 12.3
✨ Automatically selected the virtualbox driver
💿 Downloading VM boot image ...
👍 Starting control plane node minikube in cluster minikube
💾 Downloading Kubernetes v1.25.3 preload ...
🔥 Creating virtualbox VM (CPUs=2, Memory=6000MB, Disk=20000MB) ...
🐳 Preparing Kubernetes v1.25.3 on Docker 20.10.20 ...
🔎 Verifying Kubernetes components...
🌟 Enabled addons: default-storageclass, storage-provisioner
💡 kubectl not found. If you need it, try: 'minikube kubectl -- get pods -A'
🏄 Done! kubectl is now configured to use "minikube" cluster and "default" namespace by default
```


***NOTE****: An error message that reads "Unable to pick a default driver..." means that Minikube was not able to locate any one of the supported hypervisors or runtimes. The recommendation is to re-install a desired isolation tool, and ensure its executable is found in the default ****`PATH`**** of your OS.*
Check the status. With the **`minikube status`** command, we display the status of the Minikube cluster:

```
$ minikube status
minikube
type: Control Plane
host: Running
kubelet: Running
apiserver: Running
kubeconfig: Configured 
```

Stop Minikube. With the **`minikube stop`** command, we can stop Minikube:

```
$ minikube stop
✋ Stopping node "minikube" ...
🛑 1 nodes stopped.
```

## Installing Minikube on Windows

Let's learn how to install the latest Minikube release on Windows 10 with VirtualBox v7.0 specifically. This installation assumes no other isolation software is installed on our Windows workstation, such as Hyper-V, VMware Workstation, QEMU or Docker Engine, that Minikube can use as a driver.

***NOTE****: For other OS, VirtualBox, and Minikube versions, the installation steps may vary! Check the *[*_Minikube installation_*](https://kubernetes.io/docs/tasks/tools/install-minikube/)*!*

Verify the virtualization support on your Windows system (multiple output lines ending with 'Yes' indicate supported virtualization):

```
PS C:\WINDOWS\system32> systeminfo
```

Install the [_VirtualBox_](https://www.virtualbox.org/wiki/Downloads) hypervisor for 'Windows hosts'. Download and install the **`.exe`** package.

***NOTE****: You ****may**** need to _disable_ Hyper-V on your Windows host (if previously installed and used) while running VirtualBox.*
Install Minikube. We can download the latest release or a specific release from the [_Minikube release page_](https://github.com/kubernetes/minikube/releases). Once downloaded, we need to make sure it is added to our **`PATH`**. There are multiple packages available to download for Windows, found under a Minikube release. However, downloading and installing the **`.exe`** will automatically add the executable to the **`PATH`**. Let's download and install the **latest ** [**_`minikube-installer.exe`_**](https://github.com/kubernetes/minikube/releases/latest/download/minikube-installer.exe) package.

Start Minikube. We can start Minikube using the **`minikube start`** command, which bootstraps a single-node cluster with the latest stable Kubernetes version release. For a specific Kubernetes version the **`--kubernetes-version`** option can be used as such **`minikube start --kubernetes-version v1.25.1`** (where **`latest`** is default and acceptable version value, and **`stable`** is also acceptable). More advanced start options will be explored later in this chapter. Open the PowerShell using the *Run as Administrator* option and execute the following command:

```
PS C:\WINDOWS\system32> minikube start
😄 minikube v1.28.0 on Windows 10
✨ Automatically selected the virtualbox driver
💿 Downloading VM boot image ...
👍 Starting control plane node minikube in cluster minikube
💾 Downloading Kubernetes v1.25.3 preload ...
🔥 Creating virtualbox VM (CPUs=2, Memory=6000MB, Disk=20000MB) ...
🐳 Preparing Kubernetes v1.25.3 on Docker 20.10.20 ...
🔎 Verifying Kubernetes components...
🌟 Enabled addons: default-storageclass, storage-provisioner
💡 kubectl not found. If you need it, try: 'minikube kubectl -- get pods -A'
🏄 Done! kubectl is now configured to use "minikube" cluster and "default" namespace by default
```

***NOTE****: An error message that reads "Unable to pick a default driver..." means that Minikube was not able to locate any one of the supported hypervisors or runtimes. The recommendation is to install or re-install a desired isolation tool, and ensure its executable is found in the default ****`PATH`**** of your OS.*

Check the status. With the **`minikube status`** command, we display the status of the Minikube cluster. Open the PowerShell using the *Run as Administrator* option and execute the following command:

```
PS C:\WINDOWS\system32> minikube status
minikube
type: Control Plane
host: Running
kubelet: Running
apiserver: Running
kubeconfig: Configured
```

Stop Minikube. We can stop Minikube using the **`minikube stop`** command. Open the PowerShell using the *Run as Administrator* option and execute the following command:

```
PS C:\WINDOWS\system32> minikube stop
✋ Stopping node "minikube" ...
🛑 1 nodes stopped.
```


## Advanced Minikube Features (1)

Now that we have familiarized ourselves with the default **`minikube start`** command, let's dive deeper into Minikube to understand some of its more advanced features.

The **`minikube start`** by default selects a driver isolation software, such as a hypervisor or a container runtime, if one (VitualBox) or multiple are installed on the host workstation. In addition it downloads the latest Kubernetes version components. With the selected driver software it provisions a single VM named **minikube** (with hardware profile of CPUs=2, Memory=6GB, Disk=20GB) or container to host the default single-node all-in-one Kubernetes cluster. Once the node is provisioned, it bootstraps the Kubernetes control plane (with the default kubeadm tool), and it installs the latest version of the default container runtime, Docker, that will serve as a running environment for the containerized applications we will deploy to the Kubernetes cluster. The **`minikube start`** command generates a default **minikube** cluster with the specifications described above and it will store these specs so that we can restart the default cluster whenever desired. The object that stores the specifications of our cluster is called a **profile**.
 
As Minikube matures, so do its features and capabilities. With the introduction of profiles, Minikube allows users to create custom reusable clusters that can all be managed from a single command line client.

The **`minikube profile`** command allows us to view the status of all our clusters in a table formatted output. Assuming we have created only the default **minikube** cluster, we could list the properties that define the default profile with:

```
$ minikube profile list
|----------|------------|---------|----------------|------|---------|---------|-------|--------|
| Profile | VM Driver | Runtime | IP | Port | Version | Status | Nodes | Active |
|----------|------------|---------|----------------|------|---------|---------|-------|--------|
| minikube | virtualbox | docker | 192.168.59.100 | 8443 | v1.25.3 | Running | 1 | * |
|----------|------------|---------|----------------|------|---------|---------|-------|--------|
```


This table presents the columns associated with the default properties such as the profile name: minikube, the isolation driver: VirtualBox, the container runtime: Docker, the Kubernetes version: v1.25.3, the status of the cluster - running or stopped. The table also displays the number of nodes: 1 by default, the private IP address of the minikube cluster's control plane VirtualBox VM, and the secure port that exposes the API Server to cluster control plane components, agents and clients: 8443.
 
What if we desire to create several reusable clusters instead, with other drivers (Docker or Podman (still experimental on Linux)) for node isolation, or different Kubernetes versions (v1.23.3 or v1.24.4), another runtime (cri-o or containerd), and possibly 2, 3, or more nodes (if permitted by the resources of our host system)? What if we desire to further customize the cluster with a specific networking option or plugin? The **`minikube start`**  command allows us to create such custom profiles with the **`--profile`** or **`-p`** flags. Several of the isolation drivers support creation of node VMs or node containers of custom sizes as well, features that we will not explore in this course as not all are very stable at the time of this writing.
Below are a few examples of more complex **`start`** commands that allow custom clusters to be created with Minikube. 

They assume that the desired driver software (Docker and/or Podman) has been installed on the host workstation. There is no need to download the desired CNI (network plugin) or the container runtime, they will be set up and enabled by Minikube on our behalf:


```
$ minikube start --kubernetes-version=v1.23.3 \
 --driver=podman --profile minipod
$ minikube start --nodes=2 --kubernetes-version=v1.24.4 \
 --driver=docker --profile doubledocker
$ minikube start --driver=virtualbox --nodes=3 --disk-size=10g \
 --cpus=2 --memory=4g --kubernetes-version=v1.25.1 --cni=calico \
 --container-runtime=cri-o -p multivbox
$ minikube start --driver=docker --cpus=6 --memory=8g \
 --kubernetes-version="1.24.4" -p largedock
$ minikube start --driver=virtualbox -n 3 --container-runtime=containerd \
 --cni=calico -p minibox
```

Once multiple cluster profiles are available (the default **minikube** and custom **minibox**), the profiles table will look like this:

```
$ minikube profile list
|----------|------------|---------|----------------|------|---------|---------|-------|--------|
| Profile | VM Driver | Runtime | IP | Port | Version | Status | Nodes | Active |
|----------|------------|---------|----------------|------|---------|---------|-------|--------|
| minibox | virtualbox | crio | 192.168.59.101 | 8443 | v1.25.3 | Running | 3 | |
| minikube | virtualbox | docker | 192.168.59.100 | 8443 | v1.25.3 | Running | 1 | * |
|----------|------------|---------|----------------|------|---------|---------|-------|--------|
```

The **`active`** marker indicates the target cluster profile of the minikube command line tool. The target cluster can be set to **`minibox`** with the following command:

```
$ minikube profile minibox
```

The target cluster can be set to the default **`minikube`** with one of the following commands:


$ minikube profile minikube
$ minikube profile default

*Cont’d on the next page.*

## Advanced Minikube Features (2)

Most **`minikube`** commands, such as start, stop, node, etc. are profile aware, meaning that the user is required to specify the target cluster of the command, through its profile name. The default **minikube** cluster, however, can be managed without specifying its profile name. Stopping and re-starting the two clusters listed above, the **minibox** cluster and the default **minikube**: 

```
$ minikube stop -p minibox
$ minikube start -p minibox
$ minikube stop
$ minikube start
```

Additional helpful **`minikube`** commands:
To display the version of the current Minikube installation:

```
$ minikube version
minikube version: v1.28.0
commit: 986b1ebd987211ed16f8cc10aed7d2c42fc8392f
```

Completion is a helpful post installation configuration to enable the **`minikube`** command to respond to typical auto-completion mechanisms, such as completing a command in the terminal by pressing the TAB key. To enable completion for the bash shell on Ubuntu:

```
$ sudo apt install bash-completion
$ source /etc/bash_completion
$ source <(minikube completion bash)
```

If needed, also run the following command:

```
$ minikube completion bash
```

A command that allows users to list the nodes of a cluster, add new control plane or worker nodes, delete existing cluster nodes, start or stop individual nodes of a cluster:

```
$ minikube node list
minikube 192.168.59.100
$ minikube node list -p minibox
minibox 192.168.59.101
minibox-m02 192.168.59.102
minibox-m03 192.168.59.103
```

To display the cluster control plane node's IP address, or another node's IP with the **`--node`** or **`-n`** flags:

```
$ minikube ip
192.168.59.100
$ minikube -p minibox ip
192.168.59.101
$ minikube -p minibox ip -n minibox-m02
192.168.59.102
```

When a cluster configuration is no longer of use, the cluster's profile can be deleted. It is also a profile aware command - it deletes the default **minikube** cluster if no profile is specified, or a custom cluster if its profile is specified:

```
$ minikube delete
🔥 Deleting "minikube" in virtualbox ...
💀 Removed all traces of the "minikube" cluster.
$ minikube delete -p minibox
🔥 Deleting "minibox" in virtualbox ...
🔥 Deleting "minibox-m02" in virtualbox ...
🔥 Deleting "minibox-m03" in virtualbox ...
💀 Removed all traces of the "minibox" cluster.
```


For additional commands and usage options please visit the [_Minikube command line reference_](https://minikube.sigs.k8s.io/docs/commands/). 

# Chapter 7. Accessing Minikube

## Chapter Overview

In this chapter, we will learn about different methods of accessing a Kubernetes cluster.

We can use a variety of external clients or custom scripts to access our cluster for administration purposes. We will explore `kubectl` as a CLI tool to access the Minikube Kubernetes cluster, the Kubernetes Dashboard as a web-based user interface to interact with the cluster, and the curl command with proper credentials to access the cluster via APIs.

## Learning Objectives

By the end of this chapter, you should be able to:

* Compare methods to access a Kubernetes cluster.
* Access the Minikube Kubernetes cluster with kubectl.
* Access the Minikube Kubernetes cluster from the Dashboard.
* Access the Minikube Kubernetes cluster via APIs.

## Accessing Minikube

Any healthy running Kubernetes cluster can be accessed via any one of the following methods:

* Command Line Interface (CLI) tools and scripts
* Web-based User Interface (Web UI) from a web browser
* APIs from CLI or programmatically

These methods are applicable to all Kubernetes clusters.

## Accessing Minikube: Command Line Interface (CLI)

[**_`kubectl`_**](https://kubernetes.io/docs/reference/kubectl/overview/) is the Kubernetes Command Line Interface (CLI) client to manage cluster resources and applications. It is very flexible and easy to integrate with other systems, therefore it can be used standalone, or part of scripts and automation tools. Once all required credentials and cluster access points have been configured for **`kubectl`**, it can be used remotely from anywhere to access a cluster.

We will be using **`kubectl`** extensively to deploy applications, manage and configure Kubernetes resources.

## Accessing Minikube: Web-based User Interface (Web UI)

The [**_Kubernetes Dashboard_**](https://kubernetes.io/docs/tasks/access-application-cluster/web-ui-dashboard/) provides a Web-based User Interface (Web UI) to interact with a Kubernetes cluster to manage resources and containerized applications. While not as flexible as the **`kubectl`** CLI client tool, it is still a preferred tool to users who are not as proficient with the CLI.

## Accessing Minikube: APIs

The main component of the Kubernetes control plane is the **API Server**, responsible for exposing the Kubernetes APIs. The APIs allow operators and users to directly interact with the cluster. Using both CLI tools and the Dashboard UI, we can access the API server running on the control plane node to perform various operations to modify the cluster's state. The API Server is accessible through its endpoints by agents and users possessing the required credentials.
Below, we can see the representation of the HTTP API directory tree of Kubernetes:

HTTP API directory tree of Kubernetes can be divided into three independent group types:

* Core group** (`/api/v1`)
    **This group includes objects such as Pods, Services, Nodes, Namespaces, ConfigMaps, Secrets, etc.
* Named group
    This group includes objects in **`/apis/$NAME/$VERSION`** format. These different API versions imply different levels of stability and support:
    - *Alpha level* - it may be dropped at any point in time, without notice. For example, **`/apis/batch/v2alpha1`**.
    - *Beta level* - it is well-tested, but the semantics of objects may change in incompatible ways in a subsequent beta or stable release. For example, **`/apis/certificates.k8s.io/v1beta1`**.
    -* Stable level* - appears in released software for many subsequent versions. For example, **`/apis/networking.k8s.io/v1`**.
* System-wide
    This group consists of system-wide **** API endpoints, like **`/healthz`**, **`/logs`**, **`/metrics`**, **`/ui`**, etc.

We can access an API Server either directly by calling the respective API endpoints, using the CLI tools, or the Dashboard UI.

Next, we will see how we can access the Minikube Kubernetes cluster we set up in the previous chapter.

## kubectl

**`kubectl`** allows us to manage local Kubernetes clusters like the Minikube cluster, or remote clusters deployed in the cloud. It is generally installed before installing and starting Minikube, but it can also be installed after the cluster bootstrapping step.

A Minikube installation has its own kubectl CLI installed and ready to use. However, it is somewhat inconvenient to use as the **`kubectl`** command becomes a subcommand of the **`minikube`** command. Users would be required to type longer commands, such as **`minikube kubectl -- <subcommand> <object-type> <object-name> -o --option`**, instead of just **`kubectl <subcommand> <object-type> <object-name> -o --option`**. While a simple solution would be to set up an alias, the recommendation is to run the kubectl CLI tool as a standalone installation.

Once separately installed, **`kubectl`** receives its configuration automatically for Minikube Kubernetes cluster access. However, in different Kubernetes cluster setups, we may need to manually configure the cluster access points and certificates required by **`kubectl`** to securely access the cluster.

There are different methods that can be used to install **`kubectl`** listed in the [_Kubernetes documentation_](https://kubernetes.io/docs/tasks/tools/#kubectl). For best results, it is recommended to keep **`kubectl`** within one minor version of the desired Kubernetes release. Next, we will describe the kubectl CLI installation process.

Additional details about the **`kubectl`** command line client can be found in the [_kubectl book_](https://kubectl.docs.kubernetes.io/), the [_Kubernetes official documentation_](https://kubernetes.io/docs/reference/kubectl/), or its [_GitHub repository_](https://github.com/kubernetes/kubectl).

## Installing kubectl on Linux


To [_install_](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/) **`kubectl`** on Linux, follow the instructions below extracted from the official installation guide.
Download and install the latest stable **`kubectl`** binary:

```
$ curl -LO "https://dl.k8s.io/release/$(curl -L -s \
 https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
$ sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
```


Where [**_`https://dl.k8s.io/release/stable.txt`_**](https://dl.k8s.io/release/stable.txt) aims to display the latest Kubernetes stable release version.
***NOTE:**** To download and set up a specific version of ****`kubectl`**** (such as v1.25.1), issue the following command instead:*

```
$ curl -LO https://dl.k8s.io/release/v1.25.1/bin/linux/amd64/kubectl
```

The installed version can be verified with:

```
$ kubectl version --client
```

A typical helpful post-installation configuration is to enable [_shell autocompletion_](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/#enable-shell-autocompletion) for **`kubectl`**. For bash shell it can be achieved by running the following sequence of commands:

```
$ sudo apt install -y bash-completion
$ source /usr/share/bash-completion/bash_completion
$ source <(kubectl completion bash)
$ echo 'source <(kubectl completion bash)' >>~/.bashrc
```

## Installing kubectl on macOS

There are two methods to [_install_](https://kubernetes.io/docs/tasks/tools/install-kubectl-macos/) **`kubectl`** on macOS - manually and using the Homebrew package manager. Next, we present both installation methods extracted from the official installation guide.
To manually install **`kubectl`**, download the latest stable binary, make it executable and move it to the **`PATH`** with the following commands:


```
$ curl -LO "https://dl.k8s.io/release/$(curl -L -s \
 https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
$ chmod +x ./kubectl
$ sudo mv ./kubectl /usr/local/bin/kubectl
$ sudo chown root: /usr/local/bin/kubectl
```


Where [**_`https://dl.k8s.io/release/stable.txt`_**](https://dl.k8s.io/release/stable.txt) aims to display the latest Kubernetes stable release version.
***NOTE****: To download and setup a specific version of ****`kubectl`**** (such as v1.25.1), issue the following command instead:*


```
$ curl -LO https://dl.k8s.io/release/v1.25.1/bin/darwin/amd64/kubectl
```


***NOTE****: The commands above download the ****`kubectl`**** package for systems equipped with Intel processors. For newer macOS systems equipped with Apple Silicon download the required package by replacing ****`/amd64/`**** with ****`/arm64/`**** in the download commands above.*
To install **`kubectl`** with [_Homebrew package manager_](https://brew.sh/), issue the following command:


```
$ brew install kubectl
```

or

```
$ brew install kubernetes-cli
```

The installed version can be verified with:

```
$ kubectl version --client
```

A typical helpful post-installation configuration is to enable [_shell autocompletion_](https://kubernetes.io/docs/tasks/tools/install-kubectl-macos/#enable-shell-autocompletion) for **`kubectl`** on your favorite shell (bash, fish, zsh).

## Installing kubectl on Windows

To [_install_](https://kubernetes.io/docs/tasks/tools/install-kubectl-windows/) **`kubectl`**, we can download the binary directly or use **`curl`** from the CLI. Once downloaded the binary needs to be added to the **`PATH`**.

Direct download link for latest (at the time of this update) v1.25.4 binary: [**_`https://dl.k8s.io/release/v1.25.4/bin/windows/amd64/kubectl.exe`_**](https://dl.k8s.io/release/v1.25.4/bin/windows/amd64/kubectl.exe). Other versions can be downloaded simply by replacing /v1.25.4/ with the desired version.
 
***NOTE****: Obtain the latest ****`kubectl`**** stable release version number from the link below, and if needed, edit the download link for the desired binary version from above: *[***_`https://dl.k8s.io/release/stable.txt`_***](https://dl.k8s.io/release/stable.txt)*.*

Use the **`curl`** command (if installed) from the CLI:

```
curl -LO "https://dl.k8s.io/release/v1.25.4/bin/windows/amd64/kubectl.exe"
```

Once downloaded, append the **`kubectl`** binary folder to the **`PATH`**.
***NOTE****: Docker Desktop for Windows adds its own version of ****`kubectl`**** to ****`PATH`****. If you have installed Docker Desktop before, you may need to place your ****`PATH`**** entry before the one added by the Docker Desktop installer or remove the Docker Desktop's ****`kubectl`****.*
The installed version can be verified with:

```
$ kubectl version --client
```

A typical helpful post-installation configuration is to enable [_shell autocompletion_](https://kubernetes.io/docs/tasks/tools/install-kubectl-windows/#enable-shell-autocompletion) for **`kubectl`** for PowerShell.

## kubectl Configuration File

To access the Kubernetes cluster, the **`kubectl`** client needs the control plane node endpoint and appropriate credentials to be able to securely interact with the API Server running on the control plane node. While starting Minikube, the startup process creates, by default, a configuration file, **`config`**, inside the **`.kube`** directory (often referred to as the [**_`kubeconfig`_**](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/)), which resides in the user's **`home`** directory. The configuration file has all the connection details required by **`kubectl`**. By default, the **`kubectl`** binary parses this file to find the control plane node's connection endpoint, along with the required credentials. Multiple **`kubeconfig`** files can be configured with a single **`kubectl`** client. To look at the connection details, we can either display the content of the **`~/.kube/config`** file (on Linux) or run the following command (the output is redacted for readability):

```
apiVersion: v1
clusters:
- cluster:
    certificate-authority: /home/student/.minikube/ca.crt
    server: https://192.168.99.100:8443
  name: minikube
contexts:
- context:
    cluster: minikube
    user: minikube
  name: minikube
current-context: minikube
kind: Config
preferences: {}
users:
- name: minikube
  user:
    client-certificate: /home/student/.minikube/profiles/minikube/client.crt
    client-key: /home/student/.minikube/profiles/minikube/client.key
```
The kubeconfig includes the API Server's endpoint **server: [https://192.168.99.100:8443](https://192.168.99.100:8443/)** and the **minikube** user's client authentication **key** and **certificate** data.

Once **`kubectl`** is installed, we can display information about the Minikube Kubernetes cluster with the **`kubectl cluster-info`** command:

```
$ kubectl cluster-info

Kubernetes master is running at https://192.168.99.100:8443
KubeDNS is running at https://192.168.99.100:8443/api/v1/namespaces/kube-system/services/kube-dns:dns/proxy

To further debug and diagnose cluster problems, use 'kubectl cluster-info dump'.
```

You can find more details about the **`kubectl`** command line options [_here_](https://kubernetes.io/docs/reference/kubectl/overview/).
Although for the Kubernetes cluster installed by Minikube the **`~/.kube/config`** file gets created automatically, this is not the case for Kubernetes clusters installed by other tools. In other cases, the config file has to be created manually and sometimes re-configured to suit various networking and client/server setups.

## Kubernetes Dashboard

The Kubernetes Dashboard provides a web-based user interface for Kubernetes cluster management. Minikube installs the Dashboard as an addon, but it is disabled by default. Prior to using the Dashboard we are required to enable the Dashboard addon, together with the metrics-server addon, a helper addon designed to collect usage metrics from the Kubernetes cluster. To access the dashboard from Minikube, we can use the `minikube dashboard` command, which opens a new tab in our web browser displaying the Kubernetes Dashboard, but only after we list, enable required addons, and verify their state:

```
$ minikube addons list

$ minikube addons enable metrics-server

$ minikube addons enable dashboard

$ minikube addons list

$ minikube dashboard 
```

NOTE: In case the browser is not opening another tab and does not display the Dashboard as expected, verify the output in your terminal as it may display a URL for the Dashboard (together with some Error messages). If the URL is not displayed, we can request it to be displayed with the following command:

```
$ minikube dashboard --url
```

Copy and paste the displayed URL in a new tab of your browser. Depending on your terminal's features you may be able to just click or right-click the URL to open directly in the browser.

After a logout/login or a reboot of your workstation the expected behavior may be observed (where the `minikube dashboard` command directly opens a new tab in your browser displaying the Dashboard).

## APIs with 'kubectl proxy'

Issuing the `kubectl proxy` command, kubectl authenticates with the API server on the control plane node and makes services available on the default proxy port `8001`.

First, we issue the `kubectl proxy` command:

```
$ kubectl proxy

Starting to serve on 127.0.0.1:8001
```

It locks the terminal for as long as the proxy is running, unless we run it in the background (with `kubectl proxy` &). 

When `kubectl proxy` is running, we can send requests to the API over the `localhost` on the default proxy port 8001 (from another terminal, since the proxy locks the first terminal when running in foreground):

```
$ curl http://localhost:8001/

{
 "paths": [
   "/api",
   "/api/v1",
   "/apis",
   "/apis/apps",
   ......
   ......
   "/logs",
   "/metrics",
   "/openapi/v2",
   "/version"
 ]
}
```

With the above `curl` request, we requested all the API endpoints from the API server. Clicking on the link above (in the `curl` command), it will open the same listing output in a browser tab.

We can explore several path combinations with `curl` or in a browser as well, such as:

```
http://localhost:8001/api/v1

http://localhost:8001/apis/apps/v1

http://localhost:8001/healthz

http://localhost:8001/metrics
```

## APIs with Authentication

When not using the `kubectl proxy`, we need to authenticate to the API Server when sending API requests. We can authenticate by providing a Bearer Token when issuing a `curl` command, or by providing a set of keys and certificates.

A Bearer Token is an access token that can be generated by the authentication server (the API Server on the control plane node) at the client's request. Using that token, the client can securely communicate with the Kubernetes API Server without providing additional authentication details, and then, access resources. The token may need to be provided again for subsequent resource access requests. 

Let's create an access token for the `default` ServiceAccount, and grant special permission to access the root directory of the API (special permission that was not necessary when the `kubectl proxy` was used earlier). The special permission will be set through a Role Based Access Control (RBAC) policy. The policy is the `clusterrole` defined below, which is granted through the `clusterrolebinding` definition (RBAC, clusterroles, and clusterrolebindings will be discussed in a later chapter). The special permission is only needed to access the root directory of the API, but not needed to access `/api`, `/apis`, or other subdirectories:

```
$ export TOKEN=$(kubectl create token default)

$ kubectl create clusterrole api-access-root \
  --verb=get --non-resource-url=/*

$ kubectl create clusterrolebinding api-access-root \
  --clusterrole api-access-root --serviceaccount=default:default
```

Retrieve the API Server endpoint:

```
$ export APISERVER=$(kubectl config view | grep https | \
  cut -f 2- -d ":" | tr -d " ")
```

Confirm that the APISERVER stored the same IP as the Kubernetes control plane IP by issuing the following two commands and comparing their outputs:

```
$ echo $APISERVER

https://192.168.99.100:8443

$ kubectl cluster-info
```

Kubernetes control plane is running at https://192.168.99.100:8443 ...

Access the API Server using the `curl` command, as shown below:

```
$ curl $APISERVER --header "Authorization: Bearer $TOKEN" --insecure

{
 "paths": [
   "/api",
   "/api/v1",
   "/apis",
   "/apis/apps",
   ......
   ......
   "/logs",
   "/metrics",
   "/openapi/v2",
   "/version"
 ]
}
```

We can run additional `curl` commands to retrieve details about specific API groups as follows. These commands should work even without the special permission defined above and granted to the default ServiceAccount associated with the access token: 

```
$ curl $APISERVER/api/v1 --header "Authorization: Bearer $TOKEN" --insecure

$ curl $APISERVER/apis/apps/v1 --header "Authorization: Bearer $TOKEN" --insecure

$ curl $APISERVER/healthz --header "Authorization: Bearer $TOKEN" --insecure

$ curl $APISERVER/metrics --header "Authorization: Bearer $TOKEN" --insecure
```

Instead of the `access token`, we can extract the client certificate, client key, and certificate authority data from the `.kube/config` file. Once extracted, they can be encoded and then passed with a `curl` command for authentication. The new `curl` command would look similar to the example below. Keep in mind, however, that the example command below would only work with the base 64 encoded client certificate, key and certificate authority data, and it is provided only for illustrative purposes.

```
$ curl $APISERVER --cert encoded-cert --key encoded-key --cacert encoded-ca
```

# Chapter 8. Kubernetes Building Blocks

## Chapter Overview

In this chapter, we will explore the Kubernetes object model and describe some of its fundamental building blocks, such as Nodes, Namespaces, Pods, ReplicaSets, Deployments, DaemonSets, etc. We will also discuss the essential role of Labels and Selectors in a microservices-driven architecture as they logically group decoupled objects together.

## Learning Objectives

By the end of this chapter, you should be able to:

* Describe the Kubernetes object model.
* Discuss Kubernetes building blocks, e.g. Nodes, Namespaces, Pods, ReplicaSets, Deployments, DaemonSets.
* Discuss Labels and Selectors.

## Kubernetes Object Model

Kubernetes became popular due to its advanced application lifecycle management capabilities, implemented through a rich object model, representing different persistent entities in the Kubernetes cluster. Those entities describe:

* What containerized applications we are running.
* The nodes where the containerized applications are deployed.
* Application resource consumption.
* Policies attached to applications, like restart/upgrade policies, fault tolerance, ingress/egress, access control, etc.

With each object, we declare our intent, or the desired state of the object, in the **`spec`** section. The Kubernetes system manages the **`status`** section for objects, where it records the actual state of the object. At any given point in time, the Kubernetes Control Plane tries to match the object's actual state to the object's desired state. An object definition manifest must include other fields that specify the version of the API we are referencing as the **`apiVersion`**, the object type as **`kind`**, and additional data helpful to the cluster or users for accounting purposes - the **`metadata`**.
 
Examples of Kubernetes object types are Nodes, Namespaces, Pods, ReplicaSets, Deployments, DaemonSets, etc. We will explore them next.

When creating an object, the object's configuration data section from below the **`spec`** field has to be submitted to the Kubernetes API Server. The API request to create an object must have the **`spec`** section, describing the desired state, as well as other details. Although the API Server accepts object definitions in a JSON format, most often we provide such definition manifests in a YAML format which is converted by **`kubectl`** in a JSON payload and sent to the API Server.

## Nodes

[Nodes](https://kubernetes.io/docs/concepts/architecture/nodes/) are virtual identities assigned by Kubernetes to the systems part of the cluster - whether Virtual Machines, bare-metal, Containers, etc. These identities are unique to each system, and are used by the cluster for resources accounting and monitoring purposes, which helps with workload management throughout the cluster.

Each node is managed with the help of two Kubernetes node agents - kubelet and kube-proxy, while it also hosts a container runtime. The container runtime is required to run all containerized workload on the node - control plane agents and user workloads. The kubelet and kube-proxy node agents are responsible for executing all local workload management related tasks - interact with the runtime to run containers, monitor containers and node health, report any issues and node state to the API Server, and managing network traffic to containers.

Based on their pre-determined functions, there are two distinct types of nodes - control plane and worker. A typical Kubernetes cluster includes at least one control plane node, but it may include multiple control plane nodes for Highly Available (HA) control plane. In addition, the cluster includes one or mode worker nodes to provide resource redundancy in the cluster. There are cases when a single all-in-one cluster is bootstrapped as a single node on a single VM, bare-metal, or Container, when high availability and resource redundancy are not of importance. These are hybrid or mixed nodes hosting both control plane agents and user workload on the same system. Minikube allows us to bootstrap multi-node clusters with distinct, dedicated control plane nodes, however, if our host system has a limited amount of physical resources, we can easily bootstrap a single all-in-one cluster as a single node on a single VM or Container, and still be able to explore most of the topics covered in this course, with the exception of features specific to multi-node clusters, such as DaemonSets, multi node networking, etc.

Node identities are created and assigned during the cluster bootstrapping process by the tool responsible to initialize the cluster agents. Minikube is using the default kubeadm bootstrapping tool, to initialize the control plane node during the init phase and grow the cluster by adding worker or control plane nodes with the join phase.

The control plane nodes run the control plane agents, such as the API Server, Scheduler, Controller Managers, and etcd in addition to the kubelet and kube-proxy node agents, the container runtime, and add-ons for container networking, monitoring, logging, DNS, etc.

Worker nodes run the kubelet and kube-proxy node agents, the container runtime, and add-ons for container networking, monitoring, logging, DNS, etc.

## Namespaces

If multiple users and teams use the same Kubernetes cluster we can partition the cluster into virtual sub-clusters using [_Namespaces_](https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/). The names of the resources/objects created inside a Namespace are unique, but not across Namespaces in the cluster.

To list all the Namespaces, we can run the following command:

```
$ kubectl get namespaces

NAME              STATUS       AGE
default           Active       11h
kube-node-lease   Active       11h
kube-public       Active       11h
kube-system       Active       11h
```

Generally, Kubernetes creates four Namespaces out of the box: **`kube-system`**, **`kube-public`**, **`kube-node-lease`**, and **`default`**. The **`kube-system`** Namespace contains the objects created by the Kubernetes system, mostly the control plane agents. The **`default`** Namespace contains the objects and resources created by administrators and developers, and objects are assigned to it by default unless another Namespace name is provided by the user. **`kube-public`** is a special Namespace, which is unsecured and readable by anyone, used for special purposes such as exposing public (non-sensitive) information about the cluster. The newest Namespace is **`kube-node-lease`** which holds node lease objects used for node heartbeat data. Good practice, however, is to create additional Namespaces, as desired, to virtualize the cluster and isolate users, developer teams, applications, or tiers:

```
$ kubectl create namespace new-namespace-name 
```

Namespaces are one of the most desired features of Kubernetes, securing its lead against competitors, as it provides a solution to the multi-tenancy requirement of today's enterprise development teams. 

[_Resource quotas_](https://kubernetes.io/docs/concepts/policy/resource-quotas/)** **help users limit the overall resources consumed within Namespaces, while [_LimitRanges_](https://kubernetes.io/docs/concepts/policy/limit-range/) help limit the resources consumed by Containers and their enclosing objects in a Namespace. We will briefly cover quota management in a later chapter.

## Pods

A [_Pod_](https://kubernetes.io/docs/concepts/workloads/pods/) is the smallest Kubernetes workload object. It is the unit of deployment in Kubernetes, which represents a single instance of the application. A Pod is a logical collection of one or more containers, enclosing and isolating them to ensure that they:

* Are scheduled together on the same host with the Pod.
* Share the same network namespace, meaning that they share a single IP address originally assigned to the Pod.
* Have access to mount the same external storage (volumes) and other common dependencies.

Pods are ephemeral in nature, and they do not have the capability to self-heal themselves. That is the reason they are used with controllers, or operators (controllers/operators are used interchangeably), which handle Pods' replication, fault tolerance, self-healing, etc. Examples of controllers are Deployments, ReplicaSets, DaemonSets, Jobs, etc. When an operator is used to manage an application, the Pod's specification is nested in the controller's definition using the Pod Template.

Below is an example of a stand-alone Pod object's definition manifest in **`YAML`** format, without an operator:

```
apiVersion: v1
kind: Pod
metadata:
  name: nginx-pod
  labels:
    run: nginx-pod
spec:
  containers:
  - name: nginx
    image: nginx:1.22.1
    ports:
    - containerPort: 80
```
The **`apiVersion`** field must specify **`v1`** for the Pod object definition. The second required field is **`kind`** specifying the **`Pod`** object type. The third required field **`metadata`**, holds the object's name and optional labels and annotations. The fourth required field **`spec`** marks the beginning of the block defining the desired state of the Pod object - also named the **`PodSpec`**. Our Pod creates a single container running the **`nginx:1.22.1`** image pulled from a container image registry, in this case from [_Docker Hub_](https://hub.docker.com/_/nginx). The **`containerPort`** field specifies the container port to be exposed by Kubernetes resources for inter-application access or external client access - to be explored in the Services chapter. The contents of **`spec`** are evaluated for scheduling purposes, then the **kubelet** of the selected node becomes responsible for running the container image with the help of the container runtime of the node. The Pod's name and labels are used for workload accounting purposes.

## Labels

[_Labels_](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/) are **key-value pairs** attached to Kubernetes objects (e.g. Pods, ReplicaSets, Nodes, Namespaces, Persistent Volumes). Labels are used to organize and select a subset of objects, based on the requirements in place. Many objects can have the same Label(s). Labels do not provide uniqueness to objects. Controllers use Labels to logically group together decoupled objects, rather than using objects' names or IDs.
 
In the image above, we have used two Label keys: **`app`** and **`env`**. Based on our requirements, we have given different values to our four Pods. The Label **`env=dev`** logically selects and groups the top two Pods, while the Label **`app=frontend`** logically selects and groups the left two Pods. We can select one of the four Pods - bottom left, by selecting two Labels: **`app=frontend`** **AND** **`env=qa`**.

## Label Selectors

Controllers, or operators, and Services, use [_label selectors_](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors) to select a subset of objects. Kubernetes supports two types of Selectors:

* **Equality-Based Selectors
    **Equality-Based Selectors allow filtering of objects based on Label keys and values. Matching is achieved using the **`=`**, **`==`** (equals, used interchangeably), or **`!=`** (not equals) operators. For example, with **`env==dev`** or **`env=dev`** we are selecting the objects where the **`env`** Label key is set to value **`dev`**. 
* **Set-Based Selectors
    **Set-Based Selectors allow filtering of objects based on a set of values. We can use **`in`**, **`notin`** operators for Label values, and **`exist/does not exist`** operators for Label keys. For example, with **`env in (dev,qa)`** we are selecting objects where the **`env`** Label is set to either **`dev`** or **`qa`**; with **`!app`** we select objects with no Label key **`app`**.

## ReplicationControllers

Although no longer a recommended controller, a [_ReplicationController_](https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/) is a complex operator that ensures a specified number of replicas of a Pod is running at any given time, by constantly comparing the actual state with the desired state of the managed application. If there are more Pods than the desired count, the replication controller randomly terminates the number of Pods exceeding the desired count, and, if there are fewer Pods than the desired count, then the replication controller requests additional Pods to be created until the actual count matches the desired count. Generally, we do not deploy a Pod independently, as it would not be able to re-start itself if terminated in error because a Pod misses the much desired self-healing feature that Kubernetes otherwise promises. The recommended method is to use some type of an operator to run and manage Pods.
 
In addition to replication, the ReplicationController operator also supports application updates.
 
However, the default recommended controller is the [_Deployment_](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) which configures a [_ReplicaSet_](https://kubernetes.io/docs/concepts/workloads/controllers/replicaset/) controller to manage application Pods' lifecycle.

## ReplicaSets (1)

A [_ReplicaSet_](https://kubernetes.io/docs/concepts/workloads/controllers/replicaset/) is, in part, the next-generation ReplicationController, as it implements the replication and self-healing aspects of the ReplicationController. ReplicaSets support both equality- and set-based Selectors, whereas ReplicationControllers only support equality-based Selectors.
 
When a single instance of an application is running there is always the risk of the application instance crashing unexpectedly, or the entire server hosting the application crashing. If relying only on a single application instance, such a crash could adversely impact other applications, services, or clients. To avoid such possible failures, we can run in parallel multiple instances of the application, hence achieving high availability. The lifecycle of the application defined by a Pod will be overseen by a controller - the ReplicaSet. With the help of the ReplicaSet, we can scale the number of Pods running a specific application container image. Scaling can be accomplished manually or through the use of an [_autoscaler_](https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/).

Below we graphically represent a ReplicaSet, with the replica count set to 3 for a specific Pod template. Pod-1, Pod-2, and Pod-3 are identical, running the same application container image, being cloned from the same Pod template. For now, the current state matches the desired state. Keep in mind, however, that although the three Pod replicas are said to be identical - running an instance of the same application, same configuration, they are still distinct in identity - Pod name, IP address, and the Pod object ensures that the application can be individually placed on any worker node of the cluster as a result of the scheduling process. 

Below is an example of a ReplicaSet object's definition manifest in YAML format:

```
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: frontend
  labels:
    app: guestbook
    tier: frontend
spec:
  replicas: 3
  selector:
    matchLabels:
      app: guestbook
  template:
    metadata:
      labels:
        app: guestbook
    spec:
      containers:
      - name: php-redis
        image: gcr.io/google_samples/gb-frontend:v3
```

Cont’d on the next page.

## ReplicaSets (2)

Let's continue with the same ReplicaSet example and assume that one of the Pods is forced to unexpectedly terminate (due to insufficient resources, timeout, its hosting node has crashed, etc.), causing the current state to no longer match the desired state. 

The ReplicaSet detects that the current state is no longer matching the desired state and triggers a request for an additional Pod to be created, thus ensuring that the current state matches the desired state. 

ReplicaSets can be used independently as Pod controllers but they only offer a limited set of features. A set of complementary features are provided by Deployments, the recommended controllers for the orchestration of Pods. Deployments manage the creation, deletion, and updates of Pods. A Deployment automatically creates a ReplicaSet, which then creates a Pod. There is no need to manage ReplicaSets and Pods separately, the Deployment will manage them on our behalf.

We will take a closer look at Deployments next.

## Deployments (1)

[_Deployment_](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) objects provide declarative updates to Pods and ReplicaSets. The DeploymentController is part of the control plane node's controller manager, and as a controller it also ensures that the current state always matches the desired state of our running containerized application. It allows for seamless application updates and rollbacks, known as the default **RollingUpdate** strategy, through **`rollouts`** and **`rollbacks`**, and it directly manages its ReplicaSets for application scaling. It also supports a disruptive, less popular update strategy, known as **Recreate**. 

Below is an example of a Deployment object's definition manifest in YAML format:

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
  labels:
    app: nginx-deployment
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx-deployment
  template:
    metadata:
      labels:
        app: nginx-deployment
    spec:
      containers:
      - name: nginx
        image: nginx:1.20.2
        ports:
        - containerPort: 80
```

The **`apiVersion`** field is the first required field, and it specifies the API endpoint on the API server which we want to connect to; it must match an existing version for the object type defined. The second required field is **kind**, specifying the object type - in our case it is **Deployment**, but it can be Pod, ReplicaSet, Namespace, Service, etc. The third required field **metadata**, holds the object's basic information, such as name, annotations, labels, namespaces, etc. Our example shows two **spec** fields (**`spec`** and **`spec.template.spec`**). The fourth required field **`spec`** marks the beginning of the block defining the desired state of the Deployment object. In our example, we are requesting that 3 replicas, that is 3 instances of the Pod, are running at any given time. The Pods are created using the Pod Template defined in **`spec.template`**. A nested object, such as the Pod being part of a Deployment, retains its **`metadata`** and **`spec`** and loses its own **`apiVersion`** and **`kind`** - both being replaced by **`template`**. In **`spec.template.spec`**, we define the desired state of the Pod. Our Pod creates a single container running the **`nginx:1.20.2`** image from [_Docker Hub_](https://hub.docker.com/_/nginx).

Once the Deployment object is created, the Kubernetes system attaches the **`status`** field to the object and populates it with all necessary status fields.

In the following example, a new **`Deployment`** creates **`ReplicaSet`**` `**`A`** which then creates **`3 Pods`**, with each Pod Template configured to run one **`nginx:1.20.2`** container image. In this case, the **`ReplicaSet A`** is associated with **`nginx:1.20.2`** representing a state of the **`Deployment`**. This particular state is recorded as **`Revision 1`**.

In time, we need to push updates to the application managed by the Deployment object. Let's change the Pods' Template and update the container image from **`nginx:1.20.2`** to **`nginx:1.21.5`**. The **`Deployment`** triggers a new **`ReplicaSet B`** for the new container image versioned **`1.21.5`** and this association represents a new recorded state of the **`Deployment`**, **`Revision 2`**. The seamless transition between the two ReplicaSets, from **`ReplicaSet A`** with three Pods versioned **`1.20.2`** to the new **`ReplicaSet B`** with three new Pods versioned **`1.21.5`**, or from **`Revision 1`** to **`Revision 2`**, is a Deployment rolling update.

*Cont’d on the next page.*

## Deployments (2)

Once ReplicaSet B and its 3 Pods versioned 1.21.5 are ready, the Deployment starts actively managing them. However, the Deployment keeps its prior configuration states saved as Revisions which play a key factor in the rollback capability of the Deployment - returning to a prior known configuration state. In our example, if the performance of the new nginx:1.21.5 is not satisfactory, the Deployment can be rolled back to a prior Revision, in this case from Revision 2 back to Revision 1 running nginx:1.20.2 once again.

A **`rolling update`** is triggered when we update specific properties of the Pod Template for a deployment. While planned changes such as updating the container image, container port, volumes, and mounts would trigger a new Revision, other operations that are dynamic in nature, like scaling or labeling the deployment, do not trigger a rolling update, thus do not change the Revision number.

Once the rolling update has completed, the **`Deployment`** will show both **`ReplicaSets A`** and **`B`**, where **`A`** is scaled to 0 (zero) Pods, and **`B`** is scaled to 3 Pods. This is how the Deployment records its prior state configuration settings, as **`Revisions`**.
 
## DaemonSets

[DaemonSets](https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/) are operators designed to manage node agents. They resemble ReplicaSet and Deployment operators when managing multiple Pod replicas and application updates, but the DaemonSets present a distinct feature that enforces a single Pod replica to be placed per Node, on all the Nodes. In contrast, the ReplicaSet and Deployment operators by default have no control over the scheduling and placement of multiple Pod replicas on the same Node.

DaemonSet operators are commonly used in cases when we need to collect monitoring data from all Nodes, or to run a storage, networking, or proxy daemons on all Nodes, to ensure that we have a specific type of Pod running on all Nodes at all times. They are critical API resources in multi-node Kubernetes clusters. The kube-proxy agent running as a Pod on every single node in the cluster, or the Calico networking node agent implementing the Pod networking across all nodes of the cluster, are both examples of applications managed by DaemonSet operators.

Whenever a Node is added to the cluster, a Pod from a given DaemonSet is automatically placed on it. Although it ensures an automated process, the DaemonSet's Pods are placed on all cluster's Nodes by the controller itself, and not with the help of the default Scheduler. When any one Node crashes or it is removed from the cluster, the respective DaemonSet operated Pods are garbage collected. If a DaemonSet is deleted, all Pod replicas it created are deleted as well. 

The placement of DaemonSet Pods is still governed by scheduling properties which may limit its Pods to be placed only on a subset of the cluster's Nodes.  This can be achieved with the help of Pod scheduling properties such as nodeSelectors, node affinity rules, taints and tolerations. This ensures that Pods of a DaemonSet are placed only on specific Nodes, such as workers if desired. However, the default Scheduler can take over the scheduling process if a corresponding feature is enabled, accepting again node affinity rules.

Below is an example of a DaemonSet object's definition manifest in YAML format:

```
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: fluentd-agent
  namespace: kube-system
  labels:
    k8s-app: fluentd-agent
spec:
  selector:
    matchLabels:
      k8s-app: fluentd-agent
  template:
    metadata:
      labels:
        k8s-app: fluentd-agent
    spec:
      containers:
      - name: fluentd-agent
        image: quay.io/fluentd_elasticsearch/fluentd:v2.5.2
```

## Services

A containerized application deployed to a Kubernetes cluster may need to reach other such applications, or it may need to be accessible to other applications and possibly clients. This is problematic because the container does not expose its ports to the cluster's network, and it is not discoverable either. The solution would be a simple port mapping, as offered by a typical container host. However, due to the complexity of the Kubernetes framework, such a simple port mapping is not that "simple". The solution is much more sophisticated, with the involvement of the kube-proxy node agent, IP tables, routing rules, cluster DNS server, all collectively implementing a micro-load balancing mechanism that exposes a container's port to the cluster's network, even to the outside world if desired. This mechanism is called a Service, and it is the recommended method to expose any containerized application to the Kubernetes network. The benefits of the Kubernetes Service becomes more obvious when exposing a multi-replica application, when multiple containers running the same image need to expose the same port. This is where the simple port mapping of a container host would no longer work, but the Service would have no issue implementing such a complex requirement. 

This is only a brief introduction of the Kubernetes Service resource. Services, their types, configuration options, and more will be discussed in a later chapter.

# Chapter 9. Authentication, Authorization, Admission Control

## Chapter Overview

Every API request reaching the API server has to go through several control stages before being accepted by the server and acted upon. In this chapter, we will learn about the Authentication, Authorization, and Admission Control stages of the Kubernetes API access control.

## Learning Objectives

By the end of this chapter, you should be able to:

* Discuss the authentication, authorization, and access control stages of the Kubernetes API access.
* Understand the different kinds of Kubernetes users.
* Explore the different modules for authentication and authorization.

## Authentication, Authorization, and Admission Control - Overview

To access and manage Kubernetes resources or objects in the cluster, we need to access a specific API endpoint on the API server. Each access request goes through the following access control stages: 

* **Authentication**
    Authenticate a user based on credentials provided as part of API requests.
* **Authorization**
    Authorizes the API requests submitted by the authenticated user.
* **Admission Control**
    Software modules that validate and/or modify user requests.

## Authentication

Kubernetes does not have an object called *user*, nor does it store *usernames* or other related details in its object store. However, even without that, Kubernetes can use usernames for the [_Authentication_](https://kubernetes.io/docs/reference/access-authn-authz/authentication/) phase of the API access control, and to request logging as well.
Kubernetes supports two kinds of [_users_](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#users-in-kubernetes):

* **Normal Users**
    They are managed outside of the Kubernetes cluster via independent services like User/Client Certificates, a file listing usernames/passwords, Google accounts, etc.
* **Service Accounts**
    Service Accounts allow in-cluster processes to communicate with the API server to perform various operations. Most of the Service Accounts are created automatically via the API server, but they can also be created manually. The Service Accounts are tied to a particular Namespace and mount the respective credentials to communicate with the API server as Secrets.

If properly configured, Kubernetes can also support [_anonymous requests_](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#anonymous-requests), along with requests from Normal Users and Service Accounts. [_User impersonation_](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#user-impersonation) is also supported allowing a user to act as another user, a helpful feature for administrators when troubleshooting authorization policies.
For authentication, Kubernetes uses a series of [_authentication modules_](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#authentication-strategies):

* **X509 Client Certificates
    **To enable client certificate authentication, we need to reference a file containing one or more certificate authorities by passing the **`--client-ca-file=SOMEFILE`** option to the API server. The certificate authorities mentioned in the file would validate the client certificates presented by users to the API server. A demonstration video covering this topic can be found at the end of this chapter.
* **Static Token File**
    We can pass a file containing pre-defined bearer tokens with the **`--token-auth-file=SOMEFILE`** option to the API server. Currently, these tokens would last indefinitely, and they cannot be changed without restarting the API server.
* **Bootstrap Tokens**
    Tokens used for bootstrapping new Kubernetes clusters.
* **Service Account Tokens**
    Automatically enabled authenticators that use signed bearer tokens to verify requests. These tokens get attached to Pods using the Service Account Admission Controller, which allows in-cluster processes to talk to the API server.
* **OpenID Connect Tokens**
    OpenID Connect helps us connect with OAuth2 providers, such as Azure Active Directory, Salesforce, and Google, to offload the authentication to external services.
* **Webhook Token Authentication**
    With Webhook-based authentication, verification of bearer tokens can be offloaded to a remote service.
* **Authenticating Proxy**
    Allows for the programming of additional authentication logic.

We can enable multiple authenticators, and the first module to successfully authenticate the request short-circuits the evaluation. To ensure successful user authentication, we should enable at least two methods: the service account tokens authenticator and one of the user authenticators.

## Authorization (1)

After a successful authentication, users can send the API requests to perform different operations. Here, these API requests get [_authorized_](https://kubernetes.io/docs/reference/access-authn-authz/authorization/) by Kubernetes using various authorization modules that allow or deny the requests.

Some of the API request attributes that are reviewed by Kubernetes include user, group, Resource, Namespace, or API group, to name a few. Next, these attributes are evaluated against policies. If the evaluation is successful, then the request is allowed, otherwise it is denied. Similar to the Authentication step, Authorization has multiple modules, or authorizers. More than one module can be configured for one Kubernetes cluster, and each module is checked in sequence. If any authorizer approves or denies a request, then that decision is returned immediately.

**Node
**Node authorization is a special-purpose authorization mode which specifically authorizes API requests made by kubelets. It authorizes the kubelet's read operations for services, endpoints, or nodes, and writes operations for nodes, pods, and events. For more details, please review the [_Node authorization documentation_](https://kubernetes.io/docs/reference/access-authn-authz/node/).

**Attribute-Based Access Control (ABAC)
**With the ABAC authorizer, Kubernetes grants access to API requests, which combine policies with attributes. In the following example, user *bob* can only read Pods in the Namespace **`lfs158`**.

```
{
  "apiVersion": "abac.authorization.kubernetes.io/v1beta1",
  "kind": "Policy",
  "spec": {
    "user": "bob",
    "namespace": "lfs158",
    "resource": "pods",
    "readonly": true
  }
}
```

To enable ABAC mode, we start the API server with the **`--authorization-mode=ABAC`** option, while specifying the authorization policy with **`--authorization-policy-file=PolicyFile.json`**. For more details, please review the [_ABAC authorization documentation_](https://kubernetes.io/docs/reference/access-authn-authz/abac/).

**Webhook**
In Webhook mode, Kubernetes can request authorization decisions to be made by third-party services, which would return true for successful authorization, and false for failure. In order to enable the Webhook authorizer, we need to start the API server with the **`--authorization-webhook-config-file=SOME_FILENAME`** option, where **`SOME_FILENAME`** is the configuration of the remote authorization service. For more details, please see the [_Webhook mode documentation_](https://kubernetes.io/docs/reference/access-authn-authz/webhook/).

*Cont’d on the next page.*

## Authorization (2)

**Role-Based Access Control (RBAC)
**In general, with RBAC we regulate the access to resources based on the Roles of individual users. In Kubernetes, multiple Roles can be attached to subjects like users, service accounts, etc. While creating the Roles, we restrict resource access by specific operations, such as **`create`**, **`get`**, **`update`**, **`patch`**, etc. These operations are referred to as verbs. In RBAC, we can create two kinds of Roles:

* Role
    A Role grants access to resources within a specific Namespace.
* ClusterRole
    A ClusterRole grants the same permissions as Role does, but its scope is cluster-wide.

In this course, we will focus on the first kind, **Role**. Below you will find an example:

```
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: lfs158
  name: pod-reader
rules:
- apiGroups: [""] # "" indicates the core API group
  resources: ["pods"]
  verbs: ["get", "watch", "list"]
```

The manifest defines a **`pod-reader`** role, which has access only to read the Pods of **`lfs158`** Namespace. Once the role is created, we can bind it to users with a RoleBinding object. There are two kinds of RoleBindings:

* RoleBinding
    It allows us to bind users to the same namespace as a Role. We could also refer to a ClusterRole in RoleBinding, which would grant permissions to Namespace resources defined in the ClusterRole within the RoleBinding’s Namespace.
* ClusterRoleBinding
    It allows us to grant access to resources at a cluster-level and to all Namespaces.

In this course, we will focus on the first kind, **RoleBinding**. Below, you will find an example:

```
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: pod-read-access
  namespace: lfs158
subjects:
- kind: User
  name: bob
  apiGroup: rbac.authorization.k8s.io
roleRef:
  kind: Role
  name: pod-reader
  apiGroup: rbac.authorization.k8s.io
```

The manifest defines a bind between the **pod-reader** Role and user **bob**, to restrict the user to only read the Pods of the **`lfs158`** Namespace.

To enable the RBAC mode, we start the API server with the **`--authorization-mode=RBAC`** option, allowing us to dynamically configure policies. For more details, please review the [_RBAC mode_](https://kubernetes.io/docs/reference/access-authn-authz/rbac/).

## Authentication and Authorization Demo Guide (1)

This exercise guide assumes the following environment, which by default uses the certificate and key from **`/var/lib/minikube/certs/`**, and **`RBAC`** mode for authorization:

* Minikube v1.28.0
* Kubernetes v1.25.3
* containerd 1.6.8

This exercise guide was prepared for the video demonstration available at the end of this chapter.

Start Minikube:

```
$ minikube start
```

View the content of the **`kubectl`** client's configuration manifest, observing the only context **`minikube`** and the only user **`minikube`**, created by default:

```
$ kubectl config view

apiVersion: v1
clusters:
- cluster:
    certificate-authority: /home/student/.minikube/ca.crt
    extensions:
    - extension:
        last-update: Mon, 21 Nov 2022 08:14:56 CDT
        provider: minikube.sigs.k8s.io
        version: v1.25.3
      name: cluster_info
    server: https://192.168.99.100:8443
  name: minikube
contexts:
- context:
    cluster: minikube
    extensions:
    - extension:
        last-update: Mon, 21 Nov 2022 08:14:56 CDT
        provider: minikube.sigs.k8s.io
        version: v1.25.3
      name: context_info
    namespace: default
    user: minikube
  name: minikube
current-context: minikube
kind: Config
preferences: {}
users:
- name: minikube
  user:
    client-certificate: /home/student/.minikube/profiles/minikube/client.crt
    client-key: /home/student/.minikube/profiles/minikube/client.key
```

Cont’d on the next page.

## Authentication and Authorization Demo Guide (2)

Create `lfs158` namespace:

```
$ kubectl create namespace lfs158

namespace/lfs158 created
```

Create the `rbac directory` and `cd` into it:

```
$ mkdir rbac

$ cd rbac/
```

Create a new user `bob` on your workstation, and set `bob`'s password as well (the system will prompt you to enter the password twice) :

```
~/rbac$ sudo useradd -s /bin/bash bob

~/rbac$ sudo passwd bob
```

Create a `private key` for the new user bob with the `openssl` tool, then create a `certificate signing request` for bob with the same `openssl` tool:

```
~/rbac$ openssl genrsa -out bob.key 2048

Generating RSA private key, 2048 bit long modulus (2 primes)
.................................................+++++
.........................+++++
e is 65537 (0x010001)

~/rbac$ openssl req -new -key bob.key \
  -out bob.csr -subj "/CN=bob/O=learner"
```

Create a YAML definition manifest for a `certificate signing request` object, and save it with a blank value for the `request` field: 

```
~/rbac$ vim signing-request.yaml

apiVersion: certificates.k8s.io/v1
kind: CertificateSigningRequest
metadata:
  name: bob-csr
spec:
  groups:
  - system:authenticated
  request: <assign encoded value from next cat command>
  signerName: kubernetes.io/kube-apiserver-client
  usages:
  - digital signature
  - key encipherment
  - client auth
```

Cont’d on the next page.

## Authentication and Authorization Demo Guide (3)

View the `certificate`, encode it in `base64`, and assign it to the request field in the `signing-request.yaml` file:

```
~/rbac$ cat bob.csr | base64 | tr -d '\n','%'

LS0tLS1CRUd...1QtLS0tLQo=

~/rbac$ vim signing-request.yaml

apiVersion: certificates.k8s.io/v1
kind: CertificateSigningRequest
metadata:
  name: bob-csr
spec:
  groups:
  - system:authenticated
  request: LS0tLS1CRUd...1QtLS0tLQo=
  signerName: kubernetes.io/kube-apiserver-client
  usages:
  - digital signature
  - key encipherment
  - client auth
```

Create the `certificate signing request` object, then list the certificate signing request objects. It shows a `pending` state:

```
~/rbac$ kubectl create -f signing-request.yaml

certificatesigningrequest.certificates.k8s.io/bob-csr created

~/rbac$ kubectl get csr

NAME      AGE   SIGNERNAME                            REQUESTOR       CONDITION
bob-csr   12s   kubernetes.io/kube-apiserver-client   minikube-user   Pending
```

Approve the `certificate signing request` object, then list the certificate signing request objects again. It shows both `approved` and `issued` states:

```
~/rbac$ kubectl certificate approve bob-csr

certificatesigningrequest.certificates.k8s.io/bob-csr approved

~/rbac$ kubectl get csr

NAME      AGE   SIGNERNAME                            REQUESTOR       CONDITION
bob-csr   57s   kubernetes.io/kube-apiserver-client   minikube-user   Approved,Issued
```

Cont’d on the next page.

## Authentication and Authorization Demo Guide (4)

Extract the approved `certificate` from the `certificate signing request`, decode it with `base64` and save it as a `certificate` file. Then view the certificate in the newly created certificate file:

```
~/rbac$ kubectl get csr bob-csr \
  -o jsonpath='{.status.certificate}' | \
  base64 -d > bob.crt

~/rbac$ cat bob.crt

-----BEGIN CERTIFICATE-----
MIIDGzCCA...
...
...NOZRRZBVunTjK7A==
-----END CERTIFICATE-----
```

Configure the `kubectl` client's configuration manifest with user `bob`'s credentials by assigning his `key` and `certificate`: 

```
~/rbac$ kubectl config set-credentials bob \
  --client-certificate=bob.crt --client-key=bob.key

User "bob" set.
```

Create a new `context` entry in the `kubectl` client's configuration manifest for user `bob`, associated with the `lfs158` namespace in the `minikube` cluster:

```
~/rbac$ kubectl config set-context bob-context \
  --cluster=minikube --namespace=lfs158 --user=bob

Context "bob-context" created.
```

View the contents of the `kubectl` client's configuration manifest again, observing the new `context` entry `bob-context`, and the new `user` entry `bob` (the output is redacted for readability):

```
~/rbac$ kubectl config view

apiVersion: v1
clusters:
- cluster:
    certificate-authority: /home/student/.minikube/ca.crt
    ...
    server: https://192.168.99.100:8443
  name: minikube
contexts:
- context:
    cluster: minikube
    ...
    user: minikube
  name: minikube
- context:
    cluster: minikube
    namespace: lfs158
    user: bob
  name: bob-context
current-context: minikube
kind: Config
preferences: {}
users:
- name: minikube
  user:
    client-certificate: /home/student/.minikube/profiles/minikube/client.crt
    client-key: /home/student/.minikube/profiles/minikube/client.key
- name: bob
  user:
    client-certificate: /home/student/rbac/bob.crt
    client-key: /home/student/rbac/bob.key
```

Cont’d on the next page.

## Authentication and Authorization Demo Guide (5)

While in the default `minikub` context, create a new `deployment` in the `lfs158` namespace:

```
~/rbac$ kubectl -n lfs158 create deployment nginx --image=nginx:alpine

deployment.apps/nginx created
```

From the new `context bob-context` try to list pods. The attempt fails because user `bob` has no permissions configured for the `bob-context`:

```
~/rbac$ kubectl --context=bob-context get pods

Error from server (Forbidden): pods is forbidden: User "bob" cannot list resource "pods" in API group "" in the namespace "lfs158"
```

The following steps will assign a limited set of permissions to user `bob` in the `bob-context`. 

Create a YAML configuration manifest for a `pod-reader` Role object, which allows only `get, watch, list` actions/verbs in the `lfs158` namespace against `pod` resources. Then create the role `object` and list it from the default `minikube context`, but from the `lfs158` namespace:

```
~/rbac$ vim role.yaml

apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: pod-reader
  namespace: lfs158
rules:
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "watch", "list"]

~/rbac$ kubectl create -f role.yaml

role.rbac.authorization.k8s.io/pod-reader created

~/rbac$ kubectl -n lfs158 get roles

NAME         CREATED AT
pod-reader   2022-04-11T03:47:45Z
```

Cont’d on the next page.

## Authentication and Authorization Demo Guide (6)

Create a YAML configuration manifest for a `rolebinding` object, which assigns the permissions of the `pod-reader` Role to user `bob`. Then create the `rolebinding` object and list it from the default `minikube context`, but from the `lfs158` namespace:

```
~/rbac$ vim rolebinding.yaml

apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: pod-read-access
  namespace: lfs158
subjects:
- kind: User
  name: bob
  apiGroup: rbac.authorization.k8s.io
roleRef:
  kind: Role
  name: pod-reader
  apiGroup: rbac.authorization.k8s.io

~/rbac$ kubectl create -f rolebinding.yaml 

rolebinding.rbac.authorization.k8s.io/pod-read-access created

~/rbac$ kubectl -n lfs158 get rolebindings

NAME              ROLE              AGE
pod-read-access   Role/pod-reader   28s
```

Now that we have assigned permissions to `bob`, we can successfully list pods from the new context `bob-context`.

```
~/rbac$ kubectl --context=bob-context get pods

NAME                     READY   STATUS    RESTARTS   AGE
nginx-565785f75c-kl25r   1/1     Running   0          7m41s
```

## Admission Control

[_Admission controllers_](https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/) are used to specify granular access control policies, which include allowing privileged containers, checking on resource quota, etc. We force these policies using different admission controllers, like ResourceQuota, DefaultStorageClass, AlwaysPullImages, etc. They come into effect only after API requests are authenticated and authorized.

To use admission controls, we must start the Kubernetes API server with the **`--enable-admission-plugins`**, which takes a comma-delimited, ordered list of controller names:

```
--enable-admission-plugins=NamespaceLifecycle,ResourceQuota,PodSecurity,DefaultStorageClass
```

Kubernetes has some admission controllers enabled by default. For more details, please review the [_list of admission controllers_](https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/#what-does-each-admission-controller-do).

Kubernetes admission control can also be implemented though custom plugins, for a [_dynamic admission control_](https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/) method. These plugins are developed as extensions and run as admission webhooks.

# Chapter 10. Services

## Chapter Overview

Although the microservices driven architecture aims to decouple the components of an application, microservices still need agents to logically tie or group them together for management purposes, or to load balance traffic to the ones that are part of such a logical set.

In this chapter, we will learn about [_Service_](https://kubernetes.io/docs/concepts/services-networking/service/) objects used to abstract the communication between cluster internal microservices, or with the external world. A Service offers a single DNS entry for a stateless containerized application managed by the Kubernetes cluster, regardless of the number of replicas, by providing a common load balancing access point to a set of pods logically grouped and managed by a controller such as a Deployment, ReplicaSet, or DaemonSet.
 
We will also learn about the **kube-proxy** daemon, which runs on each control plane and worker node **to implement the services' configuration and to provide access to services. In addition we will discuss **service discovery** and **service types**, which decide the access scope of a service. 

## Learning Ojectives

By the end of this chapter, you should be able to:

* Discuss the benefits of logically grouping Pods with Services to access an application.
* Explain the role of the **`kube-proxy`** daemon running on each node.
* Explore the Service discovery options available in Kubernetes.
* Discuss different Service types.

## Connecting Users or Applications to Pods

To access the application, a user or another application need to connect to the Pods. As Pods are ephemeral in nature, resources like IP addresses allocated to them cannot be static. Pods could be terminated abruptly or be rescheduled based on existing requirements.

Let's take, for example, a scenario where an operator manages a set of Pods and a user/client is accessing the Pods directly by using their individual IP addresses. 

Unexpectedly, one of the Pods the user/client is accessing is terminated, and a new Pod is created by the controller. The new Pod will have a new IP address that will not be immediately known by the user/client.

To overcome this situation, Kubernetes provides a higher-level abstraction called Service, which logically groups Pods and defines a policy to access them. This grouping is achieved via Labels and Selectors.

## Services
Labels and Selectors use a **key-value** pair format. In the following graphical representation, **`app`** is the Label **key**, **`frontend`** and **`db`** are Label **values** for different Pods.

Using the selectors **`app==frontend`** and **`app==db`**, we group Pods into two logical sets: one set with 3 Pods, and one set with a single Pod.

We assign a name to the logical grouping, referred to as a **Service**. The Service name is also registered with the cluster's internal DNS service. In our example, we create two Services, **`frontend-svc`**, and **`db-svc`**, and they have the **`app==frontend`** and the **`app==db`** Selectors, respectively.

Services can expose single Pods, ReplicaSets, Deployments, DaemonSets, and StatefulSets. When exposing the Pods managed by an operator, the Service's Selector may use the same label(s) as the operator.

## Service Object Example

The following is an example of a Service object definition:

```
apiVersion: v1
kind: Service
metadata:
  name: frontend-svc
spec:
  selector:
    app: frontend
  ports:
  - protocol: TCP
    port: 80
    targetPort: 5000
```

In this example, we are creating a **`frontend-svc`** Service by selecting all the Pods that have the Label key=**`app`** set to value=**`frontend`**. By default, each Service receives an IP address routable only inside the cluster, known as **`ClusterIP`**. In our example, we have **`172.17.0.4`** and **`172.17.0.5`** as ClusterIPs assigned to our **`frontend-svc`** and **`db-svc`** Services, respectively.
 
The user/client now connects to a Service via its **`ClusterIP`**, which forwards traffic to one of the Pods attached to it. A Service provides load balancing by default while selecting the Pods for traffic forwarding.

While the Service forwards traffic to Pods, we can select the **`targetPort`** on the Pod which receives the traffic. In our example, the **`frontend-svc`** Service receives requests from the user/client on **`port:`** **`80`** and then forwards these requests to one of the attached Pods on the **`targetPort:`** **`5000`**. If the **`targetPort`** is not defined explicitly, then traffic will be forwarded to Pods on the **`port`** on which the Service receives traffic. It is very important to ensure that the value of the **`targetPort`**, which is **`5000`** in this example, matches the value of the **`containerPort`** property of the Pod **`spec`** section. 

A logical set of a Pod's IP address, along with the **`targetPort`** is referred to as a **`Service endpoint`**. In our example, the **`frontend-svc`** Service has 3 endpoints: **`10.0.1.3:5000`**, **`10.0.1.4:5000`**, and **`10.0.1.5:5000`**. Endpoints are created and managed automatically by the Service, not by the Kubernetes cluster administrator. 

## kube-proxy

Each cluster node runs a daemon called [_kube-proxy_](https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies), a node agent that watches the API server on the master node for the addition, updates, and removal of Services and endpoints. **`kube-proxy`** is responsible for implementing the Service configuration on behalf of an administrator or developer, in order to enable traffic routing to an exposed application running in Pods. In the example below, for each new Service, on each node, **`kube-proxy`** configures **iptables** rules to capture the traffic for its **`ClusterIP`** and forwards it to one of the Service's endpoints. Therefore any node can receive the external traffic and then route it internally in the cluster based on the iptables rules. When the Service is removed, **`kube-proxy`** removes the corresponding iptables rules on all nodes as well.

## Traffic Policies

The kube-proxy node agent together with the iptables implement the load-balancing mechanism of the Service when traffic is being routed to the application Endpoints. Due to restricting characteristics of the iptables this load-balancing is random by default. This means that the Endpoint Pod to receive the request forwarded by the Service will be randomly selected out of many replicas. This mechanism does not guarantee that the selected receiving Pod is the closest or even on the same node as the requester, therefore not the most efficient mechanism. Since this is the iptables supported load-balancing mechanism, if we desire better outcomes, we would need to take advantage of traffic policies.
 
[_Traffic policies_](https://kubernetes.io/docs/concepts/services-networking/service/#traffic-policies) allow users to instruct the kube-proxy on the context of the traffic routing. The two options are Cluster and Local:

* The **Cluster** option allows kube-proxy to target all ready Endpoints of the Service in the load-balancing process.
* The **Local** option, however, isolates the load-balancing process to only include the Endpoints of the Service located on the same node as the requester Pod. While this sounds like an ideal option, it does have a shortcoming - if the Service does not have a ready Endpoint on the node where the requester Pod is running, the Service will not route the request to Endpoints on other nodes to satisfy the request. 

Both the Cluster and Local options are available for requests generated internally from within the cluster, or externally from applications and clients running outside the cluster.

## Service Discovery

As Services are the primary mode of communication between containerized applications managed by Kubernetes, it is helpful to be able to discover them at runtime. Kubernetes supports two methods for discovering Services:
**Environment Variables**
As soon as the Pod starts on any worker node, the **`kubelet`** daemon running on that node adds a set of environment variables in the Pod for all active Services. For example, if we have an active Service called **`redis-master`**, which exposes port **`6379`**, and its **`ClusterIP`** is **`172.17.0.6`**, then, on a newly created Pod, we can see the following environment variables:

```
REDIS_MASTER_SERVICE_HOST=172.17.0.6
REDIS_MASTER_SERVICE_PORT=6379
REDIS_MASTER_PORT=tcp://172.17.0.6:6379
REDIS_MASTER_PORT_6379_TCP=tcp://172.17.0.6:6379
REDIS_MASTER_PORT_6379_TCP_PROTO=tcp
REDIS_MASTER_PORT_6379_TCP_PORT=6379
REDIS_MASTER_PORT_6379_TCP_ADDR=172.17.0.6
```

With this solution, we need to be careful while ordering our Services, as the Pods will not have the environment variables set for Services which are created after the Pods are created.

**DNS**
Kubernetes has an add-on for [_DNS_](https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/), which creates a DNS record for each Service and its format is **`my-svc.my-namespace.svc.cluster.local`**. Services within the same Namespace find other Services just by their names. If we add a Service **`redis-master`** in **`my-ns`** Namespace, all Pods in the same **`my-ns`** Namespace lookup the Service just by its name, **`redis-master`**. Pods from other Namespaces, such as **`test-ns`**, lookup the same Service by adding the respective Namespace as a suffix, such as **`redis-master.my-ns`** or providing the FQDN of the service as **`redis-master.my-ns.svc.cluster.local`**.

This is the most common and highly recommended solution. For example, in the previous section's image, we have seen that an internal DNS is configured, which maps our Services **`frontend-svc`** and **`db-svc`** to **`172.17.0.4`** and **`172.17.0.5`** IP addresses respectively. 

## ServiceType

While defining a Service, we can also choose its access scope. We can decide whether the Service:

* Is only accessible within the cluster.
* Is accessible from within the cluster and the external world.
* Maps to an entity which resides either inside or outside the cluster.

Access scope is decided by **`ServiceType`** property, defined when creating the Service. 

## ServiceType: ClusterIP and NodePort

**`ClusterIP`** is the default [*_ServiceType_*](https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types). A Service receives a Virtual IP address, known as its ClusterIP. This Virtual IP address is used for communicating with the Service and is accessible only from within the cluster.
 
With the [**_`NodePort`_**](https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport) *ServiceType*, in addition to a ClusterIP, a high-port, dynamically picked from the default range **`30000-32767`**, is mapped to the respective Service, from all the worker nodes. For example, if the mapped NodePort is **`32233`** for the service **`frontend-svc`**, then, if we connect to any worker node on port **`32233`**, the node would redirect all the traffic to the assigned ClusterIP - **`172.17.0.4`**. If we prefer a specific high-port number instead, then we can assign that high-port number to the NodePort from the default range when creating the Service.

The **`NodePort`** *ServiceType* is useful when we want to make our Services accessible from the external world. The end-user connects to any worker node on the specified high-port, which proxies the request internally to the ClusterIP of the Service, then the request is forwarded to the applications running inside the cluster. Let's not forget that the Service is load balancing such requests, and only forwards the request to one of the Pods running the desired application. To manage access to multiple application Services from the external world, administrators can configure a reverse proxy - an ingress, and define rules that target specific Services within the cluster.

## ServiceType: LoadBalancer

With the [**_`LoadBalancer`_**](https://kubernetes.io/docs/concepts/services-networking/service/#loadbalancer) *ServiceType*:

* NodePort and ClusterIP are automatically created, and the external load balancer will route to them.
* The Service is exposed at a static port on each worker node.
* The Service is exposed externally using the underlying cloud provider's load balancer feature.
 
**LoadBalancer**

The **`LoadBalancer`** *ServiceType* will only work if the underlying infrastructure supports the automatic creation of Load Balancers and have the respective support in Kubernetes, as is the case with the Google Cloud Platform and AWS. If no such feature is configured, the LoadBalancer IP address field is not populated, it remains in Pending state, but the Service will still work as a typical NodePort type Service.

## ServiceType: ExternalIP

A Service can be mapped to an [**_`ExternalIP`_**](https://kubernetes.io/docs/concepts/services-networking/service/#external-ips) address if it can route to one or more of the worker nodes. Traffic that is ingressed into the cluster with the ExternalIP (as destination IP) on the Service port, gets routed to one of the Service endpoints. This type of service requires an external cloud provider such as Google Cloud Platform or AWS and a Load Balancer configured on the cloud provider's infrastructure.

**ExternalIP**
 
Please note that ExternalIPs are not managed by Kubernetes. The cluster administrator has to configure the routing which maps the ExternalIP address to one of the nodes.

## ServiceType: ExternalName

[**`ExternalName`**](https://kubernetes.io/docs/concepts/services-networking/service/#externalname) is a special *ServiceType* that has no Selectors and does not define any endpoints. When accessed within the cluster, it returns a **`CNAME`** record of an externally configured Service.

The primary use case of this *ServiceType* is to make externally configured Services like **`my-database.example.com`** available to applications inside the cluster. If the externally defined Service resides within the same Namespace, using just the name **`my-database`** would make it available to other applications and Services within that same Namespace. 

## Multi-Port Services

A Service resource can expose multiple ports at the same time if required. Its configuration is flexible enough to allow for multiple groupings of ports to be defined in the manifest. This is a helpful feature when exposing Pods with one container listening on more than one port, or when exposing Pods with multiple containers listening on one or more ports.

A multi-port Service manifest is provided below: 

```
apiVersion: v1
kind: Service
metadata:
  name: my-service
spec:
  selector:
    app: myapp
  type: NodePort
  ports:
  - name: http
    protocol: TCP
    port: 8080
    targetPort: 80
    nodePort: 31080
  - name: https
    protocol: TCP
    port: 8443
    targetPort: 443
    nodePort: 31443
```

The **`my-service`** Service resource exposes Pods labeled **`app==myapp`** with possibly one container listening on ports **`80`** and **`443`**, as described by the two **`targetPort`** fields. The Service will be visible inside the cluster on its **`ClusterIP`** and ports **`8080`** and **`8443`** as described by the two **`port`** fields, and it will also be accessible to incoming requests from outside the cluster on the two **`nodePort`** fields **`31080`** and **`31443`**. When manifests describe multiple ports, they need to be named as well, for clarity, as described by the two **`name`** fields with values **`http`** and **`https`** respectively. This Service is configured to capture traffic on ports 8080 and 8443 from within the cluster, or on ports 31080 and 31443 from outside the cluster, and forward that traffic to the ports 80 and 443 respectively of the Pods running the container.

# Chapter 11. Deploying a Stand-Alone Application

## Chapter Overview

In this chapter, we will learn how to deploy an application using the Dashboard (Kubernetes WebUI) and the Command Line Interface (CLI). We will also expose the application with a NodePort type Service, and access it from outside the Minikube cluster. 

## Learning Objectives

By the end of this chapter, you should be able to:

* Deploy an application from the dashboard.
* Deploy an application from a YAML file using kubectl.
* Expose a service using NodePort.
* Access the application from outside the Minikube cluster.

## Deploying an Application Using the Dashboard (1)

Let's learn how to deploy an **`nginx`** webserver using the **`nginx`** Docker container image.

Start Minikube and verify that it is running. Run this command first:

```
$ minikube start
```

Then verify Minikube status:

```
$ minikube status

minikube
type: Control Plane
host: Running
kubelet: Running
apiserver: Running
kubeconfig: Configured
```

Start the Minikube Dashboard. To access the Kubernetes Web UI, we need to run the following command:

```
$ minikube dashboard
```

Running this command will open up a browser with the Kubernetes Web UI, which we can use to manage containerized applications. By default, the dashboard is connected to the **`default`** Namespace. So, all the operations that we will do in this chapter will be performed inside the **`default`** Namespace.

***NOTE****: In case the browser is not opening another tab and does not display the Dashboard as expected, verify the output in your terminal as it may display a link for the Dashboard (together with some Error messages). Copy and paste that link in a new tab of your browser. Depending on your terminal's features you may be able to just click or right-click the link to open directly in the browser.*

The link may look similar to:
[_http://127.0.0.1:40235/api/v1/namespaces/kubernetes-dashboard/services/http:kubernetes-dashboard:/proxy/_](http://127.0.0.1:40235/api/v1/namespaces/kubernetes-dashboard/services/http:kubernetes-dashboard:/proxy/)
Chances are that the only difference is the PORT number, which above is 40235. Your port number may be different.

After a logout/login or a reboot of your workstation the expected behavior may be observed (where the **`minikube dashboard`** command directly opens a new tab in your browser displaying the Dashboard).
*Cont’d on the next page.*

## Deploying an Application Using the Dashboard (2)

Deploy a webserver using the **`nginx`** image. From the dashboard, click on the "*+"* sign at the top right corner of the Dashboard. That will open the create interface as seen below:
 
From there, we can create an application using valid YAML/JSON configuration data, from a file, or manually from the *Create from form* tab. Click on the *Create from form* tab and provide the following application details:

* The application name is **`web-dash`**.
* The Docker image to use is **`nginx`**.
* The replica count, or the number of Pods, is 1.
* Service is External, Port 8080, Target port 80, Protocol TCP.
 
If we click on *Show Advanced Options*, we can specify options such as Labels, Namespace, etc. By default, the **`app`** Label is set to the application name. In our example **`k8s-app: web-dash`** Label is set to all objects created by this Deployment: Pods and Services (when exposed).

By clicking on the *Deploy* button, we trigger the deployment. As expected, the Deployment **`web-dash`** will create a ReplicaSet (**`web-dash-74d8bd488f`**), which will eventually create 1 Pod (**`web-dash-74d8bd488f-dwbzz`**). 

***NOTE****: Add the full URL in the Container Image field **`docker.io/library/nginx`** if any issues are encountered with the simple **`nginx`** image name (or use the **`k8s.gcr.io/nginx`** URL if it works instead).*

*Cont’d on the next page.*

## Deploying an Application Using the Dashboard (3)

Once we create the `web-dash` Deployment, we can use the resource navigation panel from the left side of the Dashboard to display details of Deployments, ReplicaSets, and Pods in the `default` Namespace. The resources displayed by the Dashboard match one-to-one resources displayed from the CLI via `kubectl`.

List the Deployments. We can list all the Deployments in the `default` Namespace using the `kubectl get deployments` command:

```
$ kubectl get deployments

NAME        READY   UP-TO-DATE   AVAILABLE   AGE
web-dash    1/1     1            1           9m
```
List the ReplicaSets. We can list all the ReplicaSets in the `default` Namespace using the `kubectl get replicasets command`:

```
$ kubectl get replicasets

NAME                   DESIRED   CURRENT   READY   AGE
web-dash-74d8bd488f    1         1         1       9m
```

List the Pods. We can list all the Pods in the `default` namespace using the `kubectl get pods` command:

```
$ kubectl get pods

NAME                          READY   STATUS    RESTARTS   AGE
web-dash-74d8bd488f-dwbzz     1/1     Running   0          9m
```

## Exploring Labels and Selectors (1)

Earlier, we have seen that labels and selectors play an important role in logically grouping a subset of objects to perform operations. Let's take a closer look at them. 

Look at Pod's details. We can look at an object's details using the `kubectl describe` command. In the following example, you can see a Pod's description:

```
$ kubectl describe pod web-dash-74d8bd488f-dwbzz

Name:           web-dash-74d8bd488f-dwbzz
Namespace:      default
Priority:       0
Node:           minikube/192.168.99.100
Start Time:     Mon, 4 Apr 2022 13:17:33 -0500
Labels:         k8s-app=web-dash
                pod-template-hash=74d8bd488f
Annotations:    <none>
Status:         Running
IP:             172.17.0.5
Controlled By:  ReplicaSet/web-dash-74d8bd488f
Containers:
  webserver:
    Container ID:   docker://96302d70903fe3b45d5ff3745a706d67d77411c5378f1f293a4bd721896d6420
    Image:          nginx
    Image ID:       docker-pullable://nginx@sha256:8d5341da24ccbdd195a82f2b57968ef5f95bc27b3c3691ace0c7d0acf5612edd
    Port:           <none>
    State:          Running
      Started:      Mon, 4 Apr 2022 13:17:33 -0500
    Ready:          True
    Restart Count:  0
...
```

The `kubectl describe` command displays many more details of a Pod. For now, however, we will focus on the `Labels` field, where we have a Label set to `k8s-app=web-dash`.

Cont’d on the next page.

## Exploring Labels and Selectors (2)

List the Pods, along with their attached Labels. With the `-L` option to the `kubectl get pods` command, we add extra columns in the output to list Pods with their attached Label keys and their values. In the following example, we are listing Pods with the Label keys `k8s-app` and `label2`:

```
$ kubectl get pods -L k8s-app,label2

NAME                         READY   STATUS    RESTARTS   AGE   K8S-APP     LABEL2
web-dash-74d8bd488f-dwbzz    1/1     Running   0          14m   web-dash   
```

All of the Pods are listed, as each Pod has the Label key `k8s-app` with value set to `web-dash`. We can see that in the K8S-APP column. As none of the Pods have the `label2` Label key, no values are listed under the `LABEL2` column.

Select the Pods with a given Label. To use a selector with the `kubectl get pods` command, we can use the -l option. In the following example, we are selecting all the Pods that have the `k8s-app` Label key set to value `web-dash`:

```
$ kubectl get pods -l k8s-app=web-dash

NAME                         READY     STATUS    RESTARTS   AGE
web-dash-74d8bd488f-dwbzz    1/1       Running   0          17m
```

In the example above, we listed all the Pods we created, as all of them have the `k8s-app` Label key set to value `web-dash`.

Try using `k8s-app=webserver` as the Selector:

```
$ kubectl get pods -l k8s-app=webserver

No resources found.
```

As expected, no Pods are listed.

## Deploying an Application Using the CLI (1)

To deploy an application using the CLI, let's first delete the Deployment we created earlier.

Delete the Deployment we created earlier. We can delete any object using the `kubectl delete` command. Next, we are deleting the `web-dash` Deployment we created earlier with the Dashboard:

```
$ kubectl delete deployments web-dash

deployment.apps "web-dash" deleted
```

Deleting a Deployment also deletes the ReplicaSet and the Pods it created:

```
$ kubectl get replicasets

No resources found.

$ kubectl get pods

No resources found.
```

Create a YAML definition manifest for a Deployment controller. Let's create the `webserver.yaml` file with the following content:

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: webserver
  labels:
    app: nginx
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:alpine
        ports:
        - containerPort: 80
```

Cont’d on the next page.

## Deploying an Application Using the CLI (2)

Using `kubectl`, we will create the Deployment from the YAML configuration file. Using the `-f` option with the `kubectl create` command, we can pass a YAML file as an object's specification, or a URL to a configuration file from the web. In the following example, we are creating a `webserver` Deployment:

```
$ kubectl create -f webserver.yaml

deployment.apps/webserver created
```

This will also create a ReplicaSet and Pods, as defined in the YAML configuration file.

```
$  kubectl get replicasets

NAME                  DESIRED   CURRENT   READY     AGE
webserver-b477df957   3         3         3         45s

$ kubectl get pods

NAME                        READY     STATUS    RESTARTS   AGE
webserver-b477df957-7lnw6   1/1       Running   0          2m
webserver-b477df957-j69q2   1/1       Running   0          2m
webserver-b477df957-xvdkf   1/1       Running   0          2m
```

As an alternative, imperatively, we can deploy the `webserver` application with the following command:

```
$  kubectl create deployment webserver --image=nginx:alpine --replicas=3 --port=80
```

## Exposing an Application (1)

In a previous chapter, we explored different ServiceTypes. With ServiceTypes we can define the access method for a Service. For a `NodePort` ServiceType, Kubernetes opens up a static port on all the worker nodes. If we connect to that port from any node, we are proxied to the ClusterIP of the Service. Next, let's use the `NodePort` ServiceType while creating a Service.

Create a `webserver-svc.yaml` file with the following content:

```
apiVersion: v1
kind: Service
metadata:
  name: web-service
  labels:
    app: nginx
spec:
  type: NodePort
  ports:
  - port: 80
    protocol: TCP
  selector:
    app: nginx 
```

Using `kubectl`, create the Service:

```
$ kubectl create -f webserver-svc.yaml

service/web-service created
```

A more direct method of creating a Service is by exposing the previously created Deployment (this method requires an existing Deployment).

Expose a Deployment with the `kubectl expose` command:

```
$ kubectl expose deployment webserver --name=web-service --type=NodePort

service/web-service exposed
```

Cont’d on the next page.

## Exposing an Application (2)

List the Services:

```
$ kubectl get services

NAME          TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)        AGE
kubernetes    ClusterIP   10.96.0.1      <none>        443/TCP        1d
web-service   NodePort    10.110.47.84   <none>        80:31074/TCP   22s
```

Our `web-service` is now created and its ClusterIP is `10.110.47.84`. In the `PORT(S)` section, we see a mapping of `80:31074`, which means that we have reserved a static port 31074 on the node. If we connect to the node on that port, our requests will be proxied to the ClusterIP on port `80`.

It is not necessary to create the Deployment first, and the Service after. They can be created in any order. A Service will find and connect Pods based on the Selector.

To get more details about the Service, we can use the `kubectl describe` command, as in the following example:

```
$ kubectl describe service web-service

Name:                     web-service
Namespace:                default
Labels:                   app=nginx
Annotations:              <none>
Selector:                 app=nginx
Type:                     NodePort
IP:                       10.110.47.84
Port:                     <unset>  80/TCP
TargetPort:               80/TCP
NodePort:                 <unset>  31074/TCP
Endpoints:                172.17.0.4:80,172.17.0.5:80,172.17.0.6:80
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
```

`web-service` uses `app=nginx` as a Selector to logically group our three Pods, which are listed as endpoints. When a request reaches our Service, it will be served by one of the Pods listed in the Endpoints section.

## Accessing an Application

Our application is running on the Minikube VM node. To access the application from our workstation, let's first get the IP address of the Minikube VM:

```
$ minikube ip

192.168.99.100
```

Now, open the browser and access the application on `192.168.99.100` at port `31074`:

We could also run the following `minikube` command which displays the application in our browser:

```
$ minikube service web-service

Opening kubernetes service default/web-service in default browser...
```

We can see the Nginx welcome page, displayed by the `webserver` application running inside the Pods created. Our requests could be served by either one of the three endpoints logically grouped by the Service since the Service acts as a Load Balancer in front of its endpoints.

In the event the above commad is not successful, and a browser window does not display the Nginx welcome page, we could run the following command instead, which returns the service URL. Pasting the URL in the browser's address bar should open the desired page:

```
$ minikube service web-service --url
```

## Liveness and Readiness Probes

While containerized applications are scheduled to run in pods on nodes across our cluster, at times the applications may become unresponsive or may be delayed during startup. Implementing [_Liveness and Readiness Probes_](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/) allows the **`kubelet`** to control the health of the application running inside a Pod's container and force a container restart of an unresponsive application. When defining both **Readiness** and **Liveness*** ***Probes**, it is recommended to allow enough time for the Readiness Probe to possibly fail a few times before a pass, and only then check the Liveness Probe. If Readiness and Liveness Probes overlap there may be a risk that the container never reaches ready state, being stuck in an infinite re-create - fail loop.

In the next few sections, we will discuss them in more detail.

## Liveness

If a container in the Pod has been running successfully for a while, but the application running inside this container suddenly stopped responding to our requests, then that container is no longer useful to us. This kind of situation can occur, for example, due to application deadlock or memory pressure. In such a case, it is recommended to restart the container to make the application available.

Rather than restarting it manually, we can use a Liveness Probe. Liveness Probe checks on an application's health, and if the health check fails, `kubelet` restarts the affected container automatically.

Liveness Probes can be set by defining:

* Liveness command
* Liveness HTTP request
* TCP Liveness probe.

## Liveness Command

In the following example, the [liveness](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/#define-a-liveness-command) command is checking the existence of a file `/tmp/healthy`:

```
apiVersion: v1
kind: Pod
metadata:
  labels:
    test: liveness
  name: liveness-exec
spec:
  containers:
  - name: liveness
    image: k8s.gcr.io/busybox
    args:
    - /bin/sh
    - -c
    - touch /tmp/healthy; sleep 30; rm -rf /tmp/healthy; sleep 600
    livenessProbe:
      exec:
        command:
        - cat
        - /tmp/healthy
      initialDelaySeconds: 15
      failureThreshold: 1
      periodSeconds: 5
```

The existence of the `/tmp/healthy` file is configured to be checked every 5 seconds using the `periodSeconds` parameter. The `initialDelaySeconds` parameter requests the kubelet to wait for 15 seconds before the first probe. When running the command line argument to the container, we will first create the `/tmp/healthy` file, and then we will remove it after 30 seconds. The removal of the file would trigger a probe failure, while the `failureThreshold` parameter set to 1 instructs kubelet to declare the container unhealthy after a single probe failure and trigger a container restart as a result.

## Liveness HTTP Request

In the following example, the kubelet sends the `HTTP GET` request to the `/healthz` endpoint of the application, on port 8080. If that returns a failure, then the kubelet will restart the affected container; otherwise, it would consider the application to be alive:

```
...
     livenessProbe:
       httpGet:
         path: /healthz
         port: 8080
         httpHeaders:
         - name: X-Custom-Header
           value: Awesome
       initialDelaySeconds: 15
       periodSeconds: 5
...
```

## TCP Liveness Probe

With [TCP Liveness Probe](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/#define-a-tcp-liveness-probe), the kubelet attempts to open the TCP Socket to the container which is running the application. If it succeeds, the application is considered healthy, otherwise the kubelet would mark it as unhealthy and restart the affected container.

```
...
    livenessProbe:
      tcpSocket:
        port: 8080
      initialDelaySeconds: 15
      periodSeconds: 5
...
```

## Readiness Probes

Sometimes, while initializing, applications have to meet certain conditions before they become ready to serve traffic. These conditions include ensuring that the dependent service is ready, or acknowledging that a large dataset needs to be loaded, etc. In such cases, we use [Readiness Probes](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/#define-readiness-probes) and wait for a certain condition to occur. Only then, the application can serve traffic.

A Pod with containers that do not report ready status will not receive traffic from Kubernetes Services.

...
    readinessProbe:
          exec:
            command:
            - cat
            - /tmp/healthy
          initialDelaySeconds: 5 
          periodSeconds: 5
...

Readiness Probes are configured similarly to Liveness Probes. Their configuration also remains the same.

Please review the [Readiness Probes](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/#define-readiness-probes) for more details.

# Chapter 12. Kubernetes Volume Management

## Chapter Overview

In today's business model, data is the most precious asset for many startups and enterprises. In a Kubernetes cluster, containers in Pods can be either data producers, data consumers, or both. While some container data is expected to be transient and is not expected to outlive a Pod, other forms of data must outlive the Pod in order to be aggregated and possibly loaded into analytics engines. Kubernetes must provide storage resources in order to provide data to be consumed by containers or to store data produced by containers. Kubernetes uses Volumes of several types and a few other forms of storage resources for container data management. In this chapter, we will talk about PersistentVolume and PersistentVolumeClaim objects, which help us attach persistent storage Volumes to Pods. 

## Learning Objectives

By the end of this chapter, you should be able to:

* Explain the need for persistent data management.
* Compare Kubernetes Volume types.
* Discuss PersistentVolumes and PersistentVolumeClaims.

## Volumes

As we know, containers running in Pods are ephemeral in nature. All data stored inside a container is deleted if the container crashes. However, the `kubelet`will restart it with a clean slate, which means that it will not have any of the old data.

To overcome this problem, Kubernetes uses [Volumes](https://kubernetes.io/docs/concepts/storage/volumes/), storage abstractions that allow various storage technologies to be used by Kubernetes and offered to containers in Pods as storage media. A Volume is essentially a mount point on the container's file system backed by a storage medium. The storage medium, content and access mode are determined by the Volume Type. 

In Kubernetes, a Volume is linked to a Pod and can be shared among the containers of that Pod. Although the Volume has the same life span as the Pod, meaning that it is deleted together with the Pod, the Volume outlives the containers of the Pod - this allows data to be preserved across container restarts.

## Volume Types

A directory which is mounted inside a Pod is backed by the underlying [_Volume Type_](https://kubernetes.io/docs/concepts/storage/volumes/#volume-types). A Volume Type decides the properties of the directory, like size, content, default access modes, etc. Some examples of Volume Types are:

* **`emptyDir`**
    An **`empty`** Volume is created for the Pod as soon as it is scheduled on the worker node. The Volume's life is tightly coupled with the Pod. If the Pod is terminated, the content of **`emptyDir`** is deleted forever. 
* **`hostPath`**
    With the **`hostPath`** Volume Type, we can share a directory between the host and the Pod. If the Pod is terminated, the content of the Volume is still available on the host.
* **`gcePersistentDisk`**
    With the **`gcePersistentDisk`** Volume Type, we can mount a [_Google Compute Engine (GCE) persistent disk_](https://cloud.google.com/compute/docs/disks/) into a Pod.
* **`awsElasticBlockStore`**
    With the **`awsElasticBlockStore`** Volume Type, we can mount an [_AWS EBS Volume_](https://aws.amazon.com/ebs/) into a Pod. 
* **`azureDisk`**
    With **`azureDisk`** we can mount a [_Microsoft Azure Data Disk_](https://docs.microsoft.com/en-us/azure/virtual-machines/linux/managed-disks-overview) into a Pod.
* **`azureFile`**
    With **`azureFile`** we can mount a [_Microsoft Azure File Volume_](https://github.com/kubernetes/examples/blob/master/staging/volumes/azure_file/README.md) into a Pod.
* **`cephfs`**
    With **`cephfs`**, an existing [_CephFS_](https://ceph.io/ceph-storage/) volume can be mounted into a Pod. When a Pod terminates, the volume is unmounted and the contents of the volume are preserved.
* **`nfs`**
    With **`nfs`**, we can mount an [_NFS_](https://en.wikipedia.org/wiki/Network_File_System) share into a Pod.
* **`iscsi`**
    With **`iscsi`**, we can mount an [_iSCSI_](https://en.wikipedia.org/wiki/ISCSI) share into a Pod.
* **`secret`**
    With the [**_`secret`_**](https://kubernetes.io/docs/concepts/configuration/secret/) Volume Type, we can pass sensitive information, such as passwords, to Pods.
* **`configMap`**
    With [**_`configMap`_**](https://kubernetes.io/docs/concepts/configuration/configmap/) objects, we can provide configuration data, or shell commands and arguments into a Pod.
* **`persistentVolumeClaim`**
    We can attach a [**_`PersistentVolume`_**](https://kubernetes.io/docs/concepts/storage/persistent-volumes/) to a Pod using a [**_`persistentVolumeClaim`_**](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#persistentvolumeclaims). 

You can learn more details about Volume Types from the [_documentation_](https://kubernetes.io/docs/concepts/storage/volumes/). 

## PersistentVolumes

In a typical IT environment, storage is managed by the storage/system administrators. The end user will just receive instructions to use the storage but is not involved with the underlying storage management.

In the containerized world, we would like to follow similar rules, but it becomes challenging, given the many Volume Types we have seen earlier. Kubernetes resolves this problem with the [_PersistentVolume (PV)_](https://kubernetes.io/docs/concepts/storage/persistent-volumes/)** **subsystem, which provides APIs for users and administrators to manage and consume persistent storage. To manage the Volume, it uses the PersistentVolume API resource type, and to consume it, it uses the PersistentVolumeClaim API resource type.

A Persistent Volume is a storage abstraction backed by several storage technologies, which could be local to the host where the Pod is deployed with its application container(s), network attached storage, cloud storage, or a distributed storage solution. A Persistent Volume is statically provisioned by the cluster administrator. 

PersistentVolumes can be [_dynamically_](https://kubernetes.io/docs/concepts/storage/dynamic-provisioning/) provisioned based on the StorageClass resource. A [_StorageClass_](https://kubernetes.io/docs/concepts/storage/storage-classes/) contains predefined provisioners and parameters to create a PersistentVolume. Using PersistentVolumeClaims, a user sends the request for dynamic PV creation, which gets wired to the StorageClass resource.

Some of the Volume Types that support managing storage using PersistentVolumes are:

* GCEPersistentDisk
* AWSElasticBlockStore
* AzureFile
* AzureDisk
* CephFS
* NFS
* iSCSI.

For a complete list, as well as more details, you can check out the [_types of Persistent Volumes_](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#types-of-persistent-volumes).

## PersistentVolumeClaims

A [_PersistentVolumeClaim (PVC)_](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#persistentvolumeclaims)** **is a request for storage by a user. Users request for PersistentVolume resources based on storage class, [_access mode_](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes), size, and optionally volume mode. There are four access modes: ReadWriteOnce (read-write by a single node), ReadOnlyMany (read-only by many nodes), ReadWriteMany (read-write by many nodes), and ReadWriteOncePod (read-write by a single pod). The optional volume modes, filesystem or block device, allow volumes to be mounted into a pod's directory or as a raw block device respectively. Once a suitable PersistentVolume is found, it is bound to a PersistentVolumeClaim. 
 
After a successful bound, the PersistentVolumeClaim resource can be used by the containers of the Pod.
 
Once a user finishes its work, the attached PersistentVolumes can be released. The underlying PersistentVolumes can then be *reclaimed* (for an admin to verify and/or aggregate data), *deleted* (both data and volume are deleted), or *recycled* for future usage (only data is deleted), based on the configured **`persistentVolumeReclaimPolicy`** property. 

To learn more, you can check out the [_PersistentVolumeClaims_](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#persistentvolumeclaims).

## Container Storage Interface (CSI)

Container orchestrators like Kubernetes, Mesos, Docker or Cloud Foundry used to have their own methods of managing external storage using Volumes. For storage vendors, it was challenging to manage different Volume plugins for different orchestrators. A maintainability challenge for Kubernetes as well, it involved in-tree storage plugins integrated into the orchestrator's source code. Storage vendors and community members from different orchestrators started working together to standardize the Volume interface - a volume plugin built using a standardized [_Container Storage Interface_](https://kubernetes.io/docs/concepts/storage/volumes/#csi) (CSI) designed to work on different container orchestrators with a variety of storage providers. Explore the [_CSI specifications_](https://github.com/container-storage-interface/spec/blob/master/spec.md) for more details.

Between Kubernetes releases v1.9 and v1.13 CSI matured from alpha to [_stable support_](https://kubernetes.io/blog/2019/01/15/container-storage-interface-ga/), which makes installing new CSI-compliant Volume drivers very easy. With CSI, third-party storage providers can [_develop solutions_](https://kubernetes-csi.github.io/docs/) without the need to add them into the core Kubernetes codebase. These solutions are CSI drivers installed only when required by cluster administrators.

## Using a Shared hostPath Volume Type Demo Guide

This exercise guide was prepared for the video demonstration available at the end of this chapter. It includes a Deployment definition manifest that can be used as a template to define other similar objects as needed. In addition to the volumes and volume mounts specified for each container, a command `stanza` allows us to run a desired command in one of the containers. The debian container's shell command line interpreter (`sh`) is invoked to run the `echo` and `sleep` commands (`-c`).

```
$ vim app-blue-shared-vol.yaml

apiVersion: apps/v1
kind: Deployment
metadata:
  creationTimestamp: null
  labels:
    app: blue-app
  name: blue-app
spec:
  replicas: 1
  selector:
    matchLabels:
      app: blue-app
  strategy: {}
  template:
    metadata:
      creationTimestamp: null
      labels:
        app: blue-app
        type: canary
    spec:
      volumes:
      - name: host-volume
        hostPath:
          path: /home/docker/blue-shared-volume
      containers:
      - image: nginx
        name: nginx
        ports:
        - containerPort: 80
        volumeMounts:
        - mountPath: /usr/share/nginx/html
          name: host-volume
      - image: debian
        name: debian
        volumeMounts:
        - mountPath: /host-vol
          name: host-volume
        command: ["/bin/sh", "-c", "echo Welcome to BLUE App! > /host-vol/index.html ; sleep infinity"]
status: {}
```

# Chapter 13. ConfigMaps and Secrets

## Chapter overview

While deploying an application, we may need to pass such runtime parameters like configuration details, permissions, passwords, keys, certificates, or tokens.

Let's assume we need to deploy ten different applications for our customers, and for each customer, we need to display the name of the company in the UI. Then, instead of creating ten different Docker images for each customer, we may just use the same template image and pass customers' names as runtime parameters. In such cases, we can use the ConfigMap API resource.

Similarly, when we want to pass sensitive information, we can use the Secret API resource. In this chapter, we will explore ConfigMaps and Secrets.

## Learning objectives

By the end of this chapter, you should be able to:

* Discuss configuration management for applications in Kubernetes using ConfigMaps.
* Share sensitive data (such as passwords) using Secrets.

## ConfigMaps

[ConfigMaps](https://kubernetes.io/docs/concepts/configuration/configmap/) allow us to decouple the configuration details from the container image. Using ConfigMaps, we pass configuration data as key-value pairs, which are consumed by Pods or any other system components and controllers, in the form of environment variables, sets of commands and arguments, or volumes. We can create ConfigMaps from literal values, from configuration files, from one or more files or directories.

## Create a ConfigMap from Literal Values

A ConfigMap can be created with the `kubectl create` command, and we can display its details using the `kubectl get` command.

Create the ConfigMap:
```
$ kubectl create configmap my-config \
  --from-literal=key1=value1 \
  --from-literal=key2=value2

configmap/my-config created 

Display the ConfigMap details for my-config:

$ kubectl get configmaps my-config -o yaml

apiVersion: v1
data:
  key1: value1
  key2: value2
kind: ConfigMap
metadata:
  creationTimestamp: 2022-04-02T07:21:55Z
  name: my-config
  namespace: default
  resourceVersion: "241345"
  selfLink: /api/v1/namespaces/default/configmaps/my-config
  uid: d35f0a3d-45d1-11e7-9e62-080027a46057
```

With the `-o` yaml option, we are requesting the `kubectl` command to produce the output in the YAML format. As we can see, the object has the `ConfigMap kind`, and it has the key-value pairs inside the data field. The name of `ConfigMap` and other details are part of the `metadata` field. 

## Create a ConfigMap from a Definition Manifest

First, we need to create a definition file with the following content:

```
apiVersion: v1
kind: ConfigMap
metadata:
  name: customer1
data:
  TEXT1: Customer1_Company
  TEXT2: Welcomes You
  COMPANY: Customer1 Company Technology Pct. Ltd.
```

where we specify the `kind`, `metadata`, and `data` fields, targeting the `v1` endpoint of the API server.

If we name the file with the definition above as `customer1-configmap.yaml`, we can then create the ConfigMap with the following command:

```
$ kubectl create -f customer1-configmap.yaml

configmap/customer1 created
```

## Create a ConfigMap from a File

First, we need to create a file `permission-reset.properties` with the following configuration data:

```
permission=read-only
allowed="true"
resetCount=3
```

We can then create the `ConfigMap` with the following command:

```
$ kubectl create configmap permission-config \
  --from-file=<path/to/>permission-reset.properties

configmap/permission-config created
```

## Use ConfigMaps Inside Pods: As Environment Variables

Inside a Container, we can retrieve the key-value data of an entire ConfigMap or the values of specific ConfigMap keys as environment variables.

In the following example all the `myapp-full-container` Container's environment variables receive the values of the `full-config-map ConfigMap` keys:

```
...
  containers:
  - name: myapp-full-container
    image: myapp
    envFrom:
    - configMapRef:
      name: full-config-map
...
```

In the following example `the myapp-specific-container` Container's environment variables receive their values from specific key-value pairs from two separate ConfigMaps, `config-map-1` and `config-map-2`:

```
...
  containers:
  - name: myapp-specific-container
    image: myapp
    env:
    - name: SPECIFIC_ENV_VAR1
      valueFrom:
        configMapKeyRef:
          name: config-map-1
          key: SPECIFIC_DATA
    - name: SPECIFIC_ENV_VAR2
      valueFrom:
        configMapKeyRef:
          name: config-map-2
          key: SPECIFIC_INFO
...
```

With the configuration presented above, we will get the `SPECIFIC_ENV_VAR1` environment variable set to the value of `SPECIFIC_DATA key` from `config-map-1` ConfigMap, and `SPECIFIC_ENV_VAR2` environment variable set to the value of `SPECIFIC_INFO key` from `config-map-2` ConfigMap.

## Use ConfigMaps Inside Pods: As Volumes

We can mount a `vol-config-map` ConfigMap as a Volume inside a Pod. For each key in the ConfigMap, a file gets created in the mount path (where the file is named with the key name) and the respective key's value becomes the content of the file:

```
...
  containers:
  - name: myapp-vol-container
    image: myapp
    volumeMounts:
    - name: config-volume
      mountPath: /etc/config
  volumes:
  - name: config-volume
    configMap:
      name: vol-config-map
...
```

For more details, please explore the documentation on using [ConfigMaps](https://kubernetes.io/docs/tasks/configure-pod-container/configure-pod-configmap/).

## Using ConfigMaps as Volumes Demo Guide

This exercise guide was prepared for the video demonstration available in this chapter. It includes an `index.html` file and a Deployment definition manifest that can be used as templates to define other similar objects as needed. The goal of the demo is to store the custom webserver `index.html` file in a ConfigMap object, which is mounted by the nginx container specified by the Pod template nested in the Deployment definition manifest.

The webserver index file:

```
$ vim index.html

<!DOCTYPE html>
<html>
<head>
<title>Welcome to GREEN App!</title>
<style>
    body {
        width: 35em;
        margin: 0 auto;
        font-family: Tahoma, Verdana, Arial, sans-serif;
        background-color: GREEN;
    }
</style>
</head>
<body>
<h1 style=\"text-align: center;\">Welcome to GREEN App!</h1>
</body>
</html>
```

The Deployment definition file:

```
$ vim web-green-with-cm.yaml

apiVersion: apps/v1
kind: Deployment
metadata:
  creationTimestamp: null
  labels:
    app: green-web
  name: green-web
spec:
  replicas: 1
  selector:
    matchLabels:
      app: green-web
  strategy: {}
  template:
    metadata:
      creationTimestamp: null
      labels:
        app: green-web
    spec:
      volumes:
      - name: web-config
        configMap:
          name: green-web-cm
      containers:
      - image: nginx
        name: nginx
        ports:
        - containerPort: 80
        volumeMounts:
        - mountPath: /usr/share/nginx/html
          name: web-config
status: {}
```

## Secrets

Let's assume that we have a *Wordpress* blog application, in which our **`wordpress`** frontend connects to the **`MySQL`** database backend using a password. While creating the Deployment for **`wordpress`**, we can include the **`MySQL`** password in the Deployment's YAML file, but the password would not be protected. The password would be available to anyone who has access to the configuration file.

In this scenario, the [_Secret_](https://kubernetes.io/docs/concepts/configuration/secret/) object can help by allowing us to encode the sensitive information before sharing it. With Secrets, we can share sensitive information like passwords, tokens, or keys in the form of key-value pairs, similar to ConfigMaps; thus, we can control how the information in a Secret is used, reducing the risk for accidental exposures. In Deployments or other resources, the Secret object is *referenced*, without exposing its content.

It is important to keep in mind that by default, the Secret data is stored as plain text inside **etcd**, therefore administrators must limit access to the API server and **etcd**. However, Secret data can be encrypted at rest while it is stored in **etcd**, but this feature needs to be enabled at the API server level. 

## Create a Secret from Literal Values

To create a Secret, we can use the `kubectl create secret` command:

```
$ kubectl create secret generic my-password \
  --from-literal=password=mysqlpassword
```

The above command would create a secret called `my-password`, which has the value of the password key set to mysqlpassword.

After successfully creating a secret we can analyze it with the `get` and `describe` commands. They do not reveal the content of the Secret. The type is listed as `Opaque`.

```
$ kubectl get secret my-password

NAME          TYPE     DATA   AGE
my-password   Opaque   1      8m

$ kubectl describe secret my-password

Name:          my-password
Namespace:     default
Labels:        <none>
Annotations:   <none>

Type Opaque

Data
====
password: 13 bytes
```

## Create a Secret from a Definition Manifest

We can create a Secret manually from a YAML definition manifest. The example manifest below is named `mypass.yaml`. There are two types of maps for sensitive information inside a Secret: `data and stringData`.

With `data` maps, each value of a sensitive information field must be encoded using `base64`. If we want to have a definition manifest for our Secret, we must first create the `base64` encoding of our password:

```
$ echo mysqlpassword | base64

bXlzcWxwYXNzd29yZAo=
```

and then use it in the definition manifest:

```
apiVersion: v1
kind: Secret
metadata:
  name: my-password
type: Opaque
data:
  password: bXlzcWxwYXNzd29yZAo=
```

Please note that base64 encoding does not mean encryption, and anyone can easily decode our encoded data:

```
$ echo "bXlzcWxwYXNzd29yZAo=" | base64 --decode

mysqlpassword
```

Therefore, make sure you do not commit a Secret's definition file in the source code.

With stringData maps, there is no need to encode the value of each sensitive information field. The value of the sensitive field will be encoded when the my-password Secret is created:

```
apiVersion: v1
kind: Secret
metadata:
  name: my-password
type: Opaque
stringData:
  password: mysqlpassword
```

Using the mypass.yaml definition file we can now create a secret with kubectl create command: 

```
$ kubectl create -f mypass.yaml

secret/my-password created
```

## Create a Secret from a File

To create a Secret from a File, we can use the kubectl `create secret` command. 

First, we encode the sensitive data and then we write the encoded data to a text file:

```
$ echo mysqlpassword | base64

bXlzcWxwYXNzd29yZAo=

$ echo -n 'bXlzcWxwYXNzd29yZAo=' > password.txt
```

Now we can create the Secret from the `password.txt` file:

```
$ kubectl create secret generic my-file-password \
  --from-file=password.txt

secret/my-file-password created
```

After successfully creating a secret we can analyze it with the `get` and `describe` commands. They do not reveal the content of the Secret. The type is listed as `Opaque`.

```
$ kubectl get secret my-file-password

NAME               TYPE     DATA   AGE 
my-file-password   Opaque   1      8m

$ kubectl describe secret my-file-password

Name:          my-file-password
Namespace:     default
Labels:        <none>
Annotations:   <none>

Type  Opaque

Data
====
password.txt:  13 bytes 
```

## Use Secrets Inside Pods: As Environment Variables

Secrets are consumed by Containers in Pods as mounted data volumes, or as environment variables, and are referenced in their entirety or specific key-values.

Below we reference only the `password` key of the `my-password` Secret and assign its value to the `WORDPRESS_DB_PASSWORD` environment variable:

```
....
spec:
  containers:
  - image: wordpress:4.7.3-apache
    name: wordpress
    env:
    - name: WORDPRESS_DB_PASSWORD
      valueFrom:
        secretKeyRef:
          name: my-password
          key: password
....
```

## Use Secrets Inside Pods: As Volumes

We can also mount a Secret as a Volume inside a Pod. The following example creates a file for each `my-password Secret` key (where the files are named after the names of the keys), the files containing the values of the respective Secret keys:

```
....
spec:
  containers:
  - image: wordpress:4.7.3-apache
    name: wordpress
    volumeMounts:
    - name: secret-volume
      mountPath: "/etc/secret-data"
      readOnly: true
  volumes:
  - name: secret-volume
    secret:
      secretName: my-password
....
```

For more details, you can explore the documentation on managing [Secrets](https://kubernetes.io/docs/tasks/configmap-secret/).

# Chapter 14. Ingress

## Chapter Overview

In an earlier chapter, we saw how we can access our deployed containerized application from the external world via Services. Among the ServiceTypes the NodePort and LoadBalancer are the most often used. For the LoadBalancer ServiceType, we need to have support from the underlying infrastructure. Even after having the support, we may not want to use it for every Service, as LoadBalancer resources are limited and they can increase costs significantly. Managing the NodePort ServiceType can also be tricky at times, as we need to keep updating our proxy settings and keep track of the assigned ports.

In this chapter, we will explore the [Ingress](https://kubernetes.io/docs/concepts/services-networking/ingress/) API resource, which represents another layer of abstraction, deployed in front of the Service API resources, offering a unified method of managing access to our applications from the external world.

## Learning Objectives

By the end of this chapter, you should be able to:

* Explain what Ingress and Ingress Controllers are.
* Understand when to use Ingress.
* Access an application from the external world using Ingress.

## Ingress (1)

With Services, routing rules are associated with a given Service. They exist for as long as the Service exists, and there are many rules because there are many Services in the cluster. If we can somehow decouple the routing rules from the application and centralize the rules management, we can then update our application without worrying about its external access. This can be done using the Ingress resource.

According to kubernetes.io,

"An Ingress is a collection of rules that allow inbound connections to reach the cluster Services".

To allow the inbound connection to reach the cluster Services, Ingress configures a Layer 7 HTTP/HTTPS load balancer for Services and provides the following:

* TLS (Transport Layer Security)
* Name-based virtual hosting
* Fanout routing
* Loadbalancing
* Custom rules.


Cont’d on the next page.

## Ingress (2)

With Ingress, users do not connect directly to a Service. Users reach the Ingress endpoint, and, from there, the request is forwarded to the desired Service. You can see an example of a **Name-Based Virtual Hosting** Ingress definition below:

```
apiVersion: networking.k8s.io/v1 
kind: Ingress
metadata:
  annotations:
    kubernetes.io/ingress.class: "nginx"
  name: virtual-host-ingress
  namespace: default
spec:
  rules:
  - host: blue.example.com
    http:
      paths:
      - backend:
          service:
            name: webserver-blue-svc
            port:
              number: 80
        path: /
        pathType: ImplementationSpecific
  - host: green.example.com
    http:
      paths:
      - backend:
          service:
            name: webserver-green-svc
            port:
              number: 80
        path: /
        pathType: ImplementationSpecific
```

In the example above, user requests to both `blue.example.com` and `green.example.com` would go to the same Ingress endpoint, and, from there, they would be forwarded to `webserver-blue-svc`, and `webserver-green-svc`, respectively.

Cont’d on the next page.

## Ingress (3)

We can also define Fanout Ingress rules, presented in the example definition and the diagram below, when requests to `example.com/blue` and `example.com/green` would be forwarded to `webserver-blue-svc` and `webserver-green-svc`, respectively:

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  annotations:
    kubernetes.io/ingress.class: "nginx"
  name: fan-out-ingress
  namespace: default
spec:
  rules:
  - host: example.com
    http:
      paths:
      - path: /blue
        backend:
          service:
            name: webserver-blue-svc
            port:
              number: 80
        pathType: ImplementationSpecific
      - path: /green
        backend:
          service:
            name: webserver-green-svc
            port:
              number: 80
        pathType: ImplementationSpecific
```
 

The Ingress resource does not do any request forwarding by itself, it merely accepts the definitions of traffic routing rules. The ingress is fulfilled by an Ingress Controller, which is a reverse proxy responsible for traffic routing based on rules defined in the Ingress resource.

## Ingress Controller

An [_Ingress Controller_](https://kubernetes.io/docs/concepts/services-networking/ingress-controllers/) is an application watching the Control Plane Node's API server for changes in the Ingress resources and updates the Layer 7 Load Balancer accordingly. Ingress Controllers are also know as Controllers, Ingress Proxy, Service Proxy, Revers Proxy, etc. Kubernetes supports an array of Ingress Controllers, and, if needed, we can also build our own. [_GCE L7 Load Balancer Controller_](https://github.com/kubernetes/ingress-gce/blob/master/README.md) and [_Nginx Ingress Controller_](https://github.com/kubernetes/ingress-nginx/blob/master/README.md) are commonly used Ingress Controllers. Other controllers are [_Contour_](https://projectcontour.io/), [_HAProxy Ingress_](https://haproxy-ingress.github.io/), [_Istio Ingress_](https://istio.io/latest/docs/tasks/traffic-management/ingress/kubernetes-ingress/), [_Kong_](https://konghq.com/), [_Traefik_](https://traefik.io/traefik/), etc. In order to ensure that the ingress controller is watching its corresponding ingress resource, the ingress resource definition manifest needs to include an ingress class annotation with the name of the desired controller **`kubernetes.io/ingress.class: "nginx"`** (for an nginx ingress controller).

Starting the Ingress Controller in Minikube is extremely simple. Minikube ships with the [_Nginx Ingress Controller_](https://www.nginx.com/products/nginx/kubernetes-ingress-controller) set up as an addon, disabled by default. It can be easily enabled by running the following command:

```
$ minikube addons enable ingress
```

## Deploy an Ingress Resource

Once the Ingress Controller is deployed, we can create an Ingress resource using the `kubectl create` command. For example, if we create a `virtual-host-ingress.yaml` file with the Name-Based Virtual Hosting Ingress rule definition that we saw in the Ingress (2) section, then we use the following command to create an Ingress resource:

```
$ kubectl create -f virtual-host-ingress.yaml
```

## Access Services Using Ingress

With the Ingress resource we just created, we should now be able to access the `webserver-blue-svc` or `webserver-green-svc` services using the `blue.example.com` and `green.example.com` URLs. As our current setup is on Minikube, we will need to update the host configuration file (`/etc/hosts` on Mac and Linux) on our workstation to the Minikube IP for those URLs. After the update, the file should look similar to:

```
$ sudo vim /etc/hosts

127.0.0.1        localhost
::1              localhost
192.168.99.100   blue.example.com green.example.com 
```

Now we can open `blue.example.com` and `green.example.com` on the browser and access each application.

# Chapter 15. Advanced Topics

## Chapter Overview

So far, in this course, we have spent most of our time understanding the basic Kubernetes concepts and simple workflows to build a solid foundation.

To support enterprise-class production workloads, Kubernetes also supports multi-node pod controllers, stateful application controllers, batch controllers, auto-scaling, resource and quota management, package management, security contexts, network and security policies, etc. In this chapter, we will briefly cover a limited number of such advanced topics, since diving into specifics would be out of scope for this course. 

## Learning Objectives

By the end of this chapter, you should be able to:

* Discuss advanced Kubernetes concepts: StatefulSets, Helm, Network Policy, Security Admission, Security Context, etc.

## Annotations

With [Annotations](https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/), we can attach arbitrary non-identifying metadata to any objects, in a key-value format:

```
"annotations": {
  "key1" : "value1",
  "key2" : "value2"
}
```

Unlike Labels, annotations are not used to identify and select objects. Annotations can be used to:

```
Store build/release IDs, PR numbers, git branch, etc.
Phone/pager numbers of people responsible, or directory entries specifying where such information can be found.
Pointers to logging, monitoring, analytics, audit repositories, debugging tools, etc.
Ingress controller information.
Deployment state and revision information.
For example, while creating a Deployment, we can add a description as seen below:

apiVersion: apps/v1
kind: Deployment
metadata:
  name: webserver
  annotations:
    description: Deployment based PoC dates 2nd Mar'2022
....
```

Annotations are displayed while describing an object:

```
$ kubectl describe deployment webserver

Name:                webserver
Namespace:           default
CreationTimestamp:   Fri, 25 Mar 2022 05:10:38 +0530
Labels:              app=webserver
Annotations:         deployment.kubernetes.io/revision=1
                     description=Deployment based PoC dates 2nd Mar'2022
... 
```

## Quota and Limits Management

When there are many users sharing a given Kubernetes cluster, there is always a concern for fair usage. A user should not take undue advantage. To address this concern, administrators can use the [_ResourceQuota_](https://kubernetes.io/docs/concepts/policy/resource-quotas/) API resource, which provides constraints that limit aggregate resource consumption per Namespace.
We can set the following types of quotas per Namespace:

* **Compute Resource Quota
    **We can limit the total sum of compute resources (CPU, memory, etc.) that can be requested in a given Namespace.
* **Storage Resource Quota
    **We can limit the total sum of storage resources (PersistentVolumeClaims, requests.storage, etc.) that can be requested.
* **Object Count Quota
    **We can restrict the number of objects of a given type (pods, ConfigMaps, PersistentVolumeClaims, ReplicationControllers, Services, Secrets, etc.).

An additional resource that helps limit resources allocation to pods and containers in a namespace, is the [_LimitRange_](https://kubernetes.io/docs/concepts/policy/limit-range/), used in conjunction with the ResourceQuota API resource. A LimitRange can:

* Set compute resources usage limits per Pod or Container in a namespace.
* Set storage request limits per PersistentVolumeClaim in a namespace.
* Set a request to limit ratio for a resource in a namespace.
* Set default requests and limits and automatically inject them into Containers' environments at runtime.

## Autoscaling

While it is fairly easy to manually scale a few Kubernetes objects, this may not be a practical solution for a production-ready cluster where hundreds or thousands of objects are deployed. We need a dynamic scaling solution which adds or removes objects from the cluster based on resource utilization, availability, and requirements. 
Autoscaling can be implemented in a Kubernetes cluster via controllers which periodically adjust the number of running objects based on single, multiple, or custom metrics. There are various types of autoscalers available in Kubernetes which can be implemented individually or combined for a more robust autoscaling solution:

* [**_Horizontal Pod Autoscaler (HPA)_**](https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/)
    HPA is an algorithm-based controller [_API resource_](https://github.com/kubernetes/community/blob/master/contributors/design-proposals/autoscaling/horizontal-pod-autoscaler.md#horizontalpodautoscaler-object) which automatically adjusts the number of replicas in a ReplicaSet, Deployment or Replication Controller based on CPU utilization.
* [**_Vertical Pod Autoscaler (VPA)_**](https://github.com/kubernetes/autoscaler/tree/master/vertical-pod-autoscaler#readme)
    VPA automatically sets Container resource requirements (CPU and memory) in a Pod and dynamically adjusts them in runtime, based on historical utilization data, current resource availability and real-time events.
* [**_Cluster Autoscaler_**](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler#cluster-autoscaler)
    Cluster Autoscaler automatically [_re-sizes the Kubernetes cluster_](https://github.com/kubernetes/autoscaler/blob/master/cluster-autoscaler/FAQ.md) when there are insufficient resources available for new Pods expecting to be scheduled or when there are underutilized nodes in the cluster.

## Jobs and CronJobs

A [Job](https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/) creates one or more Pods to perform a given task. The Job object takes the responsibility of Pod failures. It makes sure that the given task is completed successfully. Once the task is complete, all the Pods have terminated automatically. Job configuration options include:

* parallelism - to set the number of pods allowed to run in parallel;
* completions - to set the number of expected completions;
* activeDeadlineSeconds - to set the duration of the Job;
* backoffLimit - to set the number of retries before Job is marked as failed;
* ttlSecondsAfterFinished - to delay the cleanup of the finished Jobs.

Starting with the Kubernetes 1.4 release, we can also perform Jobs at scheduled times/dates with [CronJobs](https://kubernetes.io/docs/tasks/job/automated-tasks-with-cron-jobs/), where a new Job object is created about once per each execution cycle. The CronJob configuration options include:

* startingDeadlineSeconds - to set the deadline to start a Job if scheduled time was missed;
* concurrencyPolicy - to allow or forbid concurrent Jobs or to replace old Jobs with new ones. 

## StatefulSets

The [StatefulSet](https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/) controller is used for stateful applications which require a unique identity, such as name, network identifications, or strict ordering. For example, `MySQL cluster`, `etcd cluster`.

The `StatefulSet` controller provides identity and guaranteed ordering of deployment and scaling to Pods. However, the StatefulSet controller has very strict Service and Storage Volume dependencies that make it challenging to configure and deploy. It also supports scaling, rolling updates, and rollbacks.

## Custom Resources

In Kubernetes, a **resource** is an API endpoint which stores a collection of API objects. For example, a Pod resource contains all the Pod objects.

Although in most cases existing Kubernetes resources are sufficient to fulfill our requirements, we can also create new resources using **custom resources**. With custom resources, we don't have to modify the Kubernetes source.

Custom resources are dynamic in nature, and they can appear and disappear in an already running cluster at any time.
To make a resource declarative, we must create and install a **custom controller**, which can interpret the resource structure and perform the required actions. Custom controllers can be deployed and managed in an already running cluster. 
There are two ways to add custom resources:

* [**_Custom Resource Definitions (CRDs)_**](https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/custom-resources/)
    This is the easiest way to add custom resources and it does not require any programming knowledge. However, building the custom controller would require some programming. 
* [**_API Aggregation_**](https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/)
    For more fine-grained control, we can write API Aggregators. They are subordinate API services which sit behind the primary API Server. The primary API Server acts as a proxy for all incoming API requests - it handles the ones based on its capabilities and proxies over the other requests meant for the subordinate API services.

## Kubernetes Federation

With [Kubernetes Cluster Federation](https://github.com/kubernetes-sigs/kubefed/blob/master/README.md) we can manage multiple Kubernetes clusters from a single control plane. We can sync resources across the federated clusters and have cross-cluster discovery. This allows us to perform Deployments across regions, access them using a global DNS record, and achieve High Availability. 

Although still a Beta feature, the Federation is very useful when we want to build a hybrid solution, with one cluster running inside our private datacenter and another one in the public cloud, allowing us to avoid provider lock-in. We can also assign weights for each cluster in the Federation, to distribute the load based on custom rules.

## Security Contexts and Pod Security Admission

At times we need to define specific privileges and access control settings for Pods and Containers. [_Security Contexts_](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/) allow us to set Discretionary Access Control for object access permissions, privileged running, capabilities, security labels, etc. However, their effect is limited to the individual Pods and Containers where such context configuration settings are incorporated in the **`spec`** section.
 
In order to apply security settings to multiple Pods and Containers cluster-wide, we can use the [_Pod Security Admission_](https://kubernetes.io/docs/concepts/security/pod-security-admission/), a built-in admission controller for Pod Security that is enabled by default in the API Server. It can enforce the three [_Pod Security Standards_](https://kubernetes.io/docs/concepts/security/pod-security-standards/) at namespace level, by automating the security context restriction to pods when they are deployed. Each Pod Security Standard profile, *privileged*, *baseline*, and *restricted*, defines a level of security that ranges from entirely unrestricted (for privileged workload), to enforcing pod hardening best practices (for security critical applications and less trusted users). The security levels are defined by sets of pod security context controls that are pre-determined for each standard.

## Network Policies

Kubernetes was designed to allow all Pods to communicate freely, without restrictions, with all other Pods in cluster Namespaces. In time it became clear that it was not an ideal design, and mechanisms needed to be put in place in order to restrict communication between certain Pods and applications in the cluster Namespace. [_Network Policies_](https://kubernetes.io/docs/concepts/services-networking/network-policies/) are sets of rules which define how Pods are allowed to talk to other Pods and resources inside and outside the cluster. Pods not covered by any Network Policy will continue to receive unrestricted traffic from any endpoint.
 
Network Policies are very similar to typical Firewalls. They are designed to protect mostly assets located inside the Firewall but can restrict outgoing traffic as well based on sets of rules and policies.
 
The Network Policy API resource specifies **`podSelectors`**, *Ingress* and/or *Egress* **`policyTypes`**, and rules based on source and destination **`ipBlocks`** and **`ports`**. Very simplistic default allow or default deny policies can be defined as well. As a good practice, it is recommended to define a default deny policy to block all traffic to and from the Namespace, and then define sets of rules for specific traffic to be allowed in and out of the Namespace.
 
Let's keep in mind that not all the networking solutions available for Kubernetes support Network Policies. Review the Pod-to-Pod Communication section from the Kubernetes Architecture chapter if needed. By default, Network Policies are namespaced API resources, but certain network plugins provide additional features so that Network Policies can be applied cluster-wide. 

## Monitoring, Logging, and Troubleshooting

In Kubernetes, we have to collect resource usage data by Pods, Services, nodes, etc., to understand the overall resource consumption and to make decisions for scaling a given application. Two popular Kubernetes *monitoring* solutions are the Kubernetes Metrics Server and Prometheus. 

* **Metrics Server** **** 
    [_Metrics Server_](https://kubernetes.io/docs/tasks/debug-application-cluster/resource-metrics-pipeline/#metrics-server) is a cluster-wide aggregator of resource usage data - a relatively new feature in Kubernetes.
* **Prometheus
    **[_Prometheus_](https://prometheus.io/), now part of [_CNCF_](https://www.cncf.io/) (Cloud Native Computing Foundation), can also be used to scrape the resource usage from different Kubernetes components and objects. Using its client libraries, we can also instrument the code of our application. 

Another important aspect for troubleshooting and debugging is *logging*, in which we collect the logs from different components of a given system. In Kubernetes, we can collect logs from different cluster components, objects, nodes, etc. Unfortunately, however, Kubernetes does not provide cluster-wide logging by default, therefore third party tools are required to centralize and aggregate cluster logs. A popular method to collect logs is using [_Elasticsearch_](https://docs.fluentd.org/output/elasticsearch) together with [_Fluentd_](http://www.fluentd.org/) with custom configuration as an agent on the nodes. Fluentd is an open source data collector, which is also part of CNCF.
The third-party troubleshooting tools are addressing a shortcoming of Kubernetes with regards to its logging capability. Although we can extract container logs from the cluster, we are limited only to logs of currently running containers, and in the case of several consecutive container restarts due to failures - the logs of the very last failed container (using the **`-p`** or **`--previous`** flags). The logs can be displayed for a single container pod or a specific container of a multi-container pod (using the **`-c`** flag):

```
$ kubectl logs pod-name
$ kubectl logs pod-name -c container-name
$ kubectl logs pod-name -c container-name -p
```
In addition, a user can run a custom command in a running container of a pod, or interact with the running container from the terminal (using the **`-it`** flag and invoking the shell command line interpreter of the container):

```
$ kubectl exec pod-name -- ls -la /
$ kubectl exec pod-name -c container-name -- env
$ kubectl exec pod-name -c container-name -it -- /bin/sh
```
There are scenarios when the pods of an application do not reach the expected running state, visible in the output of the kubectl get pods command. In order to discover what prevents the pod from running - whether a missing dependency, container image or runtime issue, we can view the events for the entire cluster or for a specific pod in the output of the **`describe`** command:

```
$ kubectl get events
$ kubectl describe pod pod-name
```
## Helm

To deploy a complex application, we use a large number of Kubernetes manifests to define API resources such as Deployments, Services, PersistentVolumes, PersistentVolumeClaims, Ingress, or ServiceAccounts. It can become counter productive to deploy them one by one. We can bundle all those manifests after templatizing them into a well-defined format, along with other metadata. Such a bundle is referred to as *Chart*. These Charts can then be served via repositories, such as those that we have for **`rpm`** and **`deb`** packages.
 
[_Helm_](https://helm.sh/) is a package manager (analogous to **`yum`** and **`apt`** for Linux) for Kubernetes, which can install/update/delete those Charts in the Kubernetes cluster.

Helm is a CLI client that may run side-by-side with **`kubectl`** on our workstation, that also uses **kubeconfig** to securely communicate with the Kubernetes API server.
 
The **helm** client queries the Chart repositories for Charts based on search parameters, downloads a desired Chart, and then it requests the API server to deploy in the cluster the resources defined in the Chart. Charts submitted for Kubernetes are available [_here_](https://artifacthub.io/).
 
Additional information about helm Charts can be found [_here_](https://helm.sh/docs/topics/charts/). 

## Service Mesh

Service Mesh is a third party solution alternative to the Kubernetes native application connectivity and exposure achieved with Services paired with Ingress Controllers. Service Mesh tools are gaining popularity especially with larger organizations managing larger, more dynamic Kubernetes clusters. These third party solutions introduce features such as service discovery, multi-cloud routing, and traffic telemetry.
A Service Mesh is an implementation that relies on a proxy component part of the Data Plane, which is then managed through a Control Plane. The Control Plane runs agents responsible for the service discovery, telemetry, load balancing, network policy, and gateway. The Data Plane proxy component is typically injected into Pods, and it is responsible for handling all Pod-to-Pod communication, while maintaining a constant communication with the Control Plane of the Service Mesh.
Several implementations of Service Mesh are:

* [_Consul_](https://www.consul.io/) by [_HashiCorp_](https://www.hashicorp.com/)
* [_Istio_](https://istio.io/) is one of the most popular service mesh solutions, backed by Google, IBM and Lyft
* [_Kuma_](https://kuma.io/) by [_Kong_](https://konghq.com/)
* [_Linkerd_](https://linkerd.io/) a [_CNCF_](https://www.cncf.io/) project
* [_Cilium_](https://cilium.io/) an eBPF-based service mesh
* [_Traefik Mesh_](https://traefik.io/traefik-mesh/) by [_Containous_](https://traefik.io/), the developers of [_Traefik_](https://traefik.io/traefik/) proxy ingress controller
* [_Tanzu Service Mesh_](https://tanzu.vmware.com/service-mesh) by [_VMware_](https://www.vmware.com/).

## Application Deployment Strategies

A method presented earlier for new application release rollouts was the Rolling Update mechanism supported by the Deployment operator. The Rolling Update mechanism, and its reverse - the Rollback, are practical methods to manage application updates by allowing one single controller, the Deployment, to handle all the work it involves. However, while transitioning between the old and the new versions of the application replicas, the Service exposing the Deployment eventually forwards traffic to all replicas, old and new, without any possibility for the default Service to isolate a subset of the Deployment's replicas. Because of the traffic routing challenges these update mechanisms introduce, many users may steer away from the one Deployment and one Service model, and embrace more complex deployment mechanism alternatives. 

The Canary strategy runs two application releases simultaneously managed by two independent Deployment controllers, both exposed by the same Service. The users can manage the amount of traffic each Deployment is exposed to by separately scaling up or down the two Deployment controllers, thus increasing or decreasing the number of their replicas receiving traffic. 

The Blue/Green strategy runs the same application release or two releases of the application on two isolated environments, but only one of the two environments is actively receiving traffic, while the second environment is idle, or may undergo rigorous testing prior to shifting traffic to it. This strategy would also require two independent Deployment controllers, each exposed by their dedicated Services, however, a traffic shifting mechanism is also required. Typically, the traffic shifting can be implemented with the use of an Ingress.

# Chapter 16. Kubernetes Community

## Chapter overview

Just as with any other open source project, the community plays a vital role in the development of Kubernetes. The community decides the roadmap of the projects and works towards it. The community becomes engaged in different online and offline forums, like Meetups, Slack, Weekly meetings, etc. In this chapter, we will explore the Kubernetes community and see how you can become a part of it, too. 

## Learning Obejectives

By the end of this chapter, you should be able to:

* Understand the importance of the Kubernetes community.
* Learn about the different channels to interact with the Kubernetes community.
* List major CNCF events.

## Kubernetes Community

With over [_85K GitHub stars_](https://github.com/kubernetes/kubernetes/), Kubernetes is one of the most popular open source projects. The community members not only help with the source code, but they also help with sharing the knowledge. The community engages in both online and offline activities.

Currently, there is a project called [_K8s Port_](https://k8sport.wufoo.com/forms/sign-up/), which recognizes and rewards community members for their contributions to Kubernetes. This contribution can be in the form of code, attending and speaking at meetups, answering questions on Stack Overflow, etc.

Next, we will review some of the mediums used by the Kubernetes community.

## Weekly Meetings and Meetup Groups

**Weekly Meetings
**A weekly community meeting happens using video conference tools. You can request a calendar invite from [_here_](https://groups.google.com/forum/#!forum/kubernetes-community-video-chat).

**Meetup Groups
**There are many [_meetup groups_](https://www.meetup.com/topics/kubernetes/) around the world, where local community members meet at regular intervals to discuss Kubernetes and its ecosystem.

**Kubernetes Community Days**
With [_events_](https://kubernetescommunitydays.org/) in Amsterdam, Bengaluru, Berlin, London, Montreal, Munich, Portland, Paris, Quebec, Tokyo, and Washington DC. 

**Cloud Native Community Groups**
With [_events_](https://community.cncf.io/) in Canada, Mexico, and United States.
There are some online meetup groups as well, where community members can meet virtually.

## Slack Channels and Mailing Lists


**Slack Channels
**Community members are very active on the [_Kubernetes Slack_](https://kubernetes.slack.com/). There are different channels based on topics, and anyone can join and participate in the discussions. You can discuss with the Kubernetes team on the **`#kubernetes-users`** channel.
 
**Mailing Lists
**There are Kubernetes [_users_](https://groups.google.com/forum/#!forum/kubernetes-users) and [_developers_](https://groups.google.com/forum/#!forum/kubernetes-dev) mailing lists, which can be joined by anybody interested.

## SIGs and Stack Overflow

**Special Interest Groups
**Special Interest Groups (SIGs) focus on specific parts of the Kubernetes project, like scheduling, authorization, networking, documentation, etc. Each group may have a different workflow, based on its specific requirements. A list with all the current SIGs can be found [_here_](https://github.com/kubernetes/community/blob/master/sig-list.md).

Depending on the need, a [_new SIG can be created_](https://github.com/kubernetes/community/blob/master/sig-wg-lifecycle.md).

**Stack Overflow
**Besides Slack and mailing lists, community members can get support from [_Stack Overflow_](https://stackoverflow.com/questions/tagged/kubernetes), as well. Stack Overflow is an online environment where you can post questions that you cannot find an answer for. The Kubernetes team also monitors the posts tagged Kubernetes.

## CNCF Events

CNCF organizes numerous international conferences on Kubernetes, as well as other CNCF projects. For more information about these events, please click [here](https://www.cncf.io/community/kubecon-cloudnativecon-events/).
Three of the major conferences it organizes are:

* KubeCon + CloudNativeCon Europe
* KubeCon + CloudNativeCon North America
* KubeCon + CloudNativeCon China.

Other events are:

* Envoycon
* Open Source Summit Europe
* Open Source Summit Japan. 












