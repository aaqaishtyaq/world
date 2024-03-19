+++
title = "Firecracker Dev Machine Setup (GCP)"
date = 2023-01-26
+++

VM on Google Compute Engine (GCE) supports nested virtualisation, unlike AWS, and allows to run KVM.
We can create a VM on GCE to run and test Firecracker microVM.

### Here are the steps

1. Setup GCP Project and Zone

    ```bash
    FC_PROJECT=your_name-firecracker
    FC_REGION=us-east1
    FC_ZONE=us-east1-b
    ```

    ```bash
    gcloud config set project ${FC_PROJECT}
    gcloud config set compute/region ${FC_REGION}
    gcloud config set compute/zone ${FC_ZONE}
    ```

2. Create a VM image (Machine Image) able to run KVM

    ```bash
    FC_VDISK=disk-ub22
    FC_IMAGE=ub22-nested-kvm

    gcloud compute disks create ${FC_VDISK} \
      --image-project ubuntu-os-cloud \
      --image-family ubuntu-2204-lts

    gcloud compute images create ${FC_IMAGE} \
      --source-disk ${FC_VDISK} \
      --source-disk-zone ${FC_ZONE} \
      --licenses "https://www.googleapis.com/compute/v1/projects/vm-options/global/licenses/enable-vmx"
    ```

3. Create the VM

    ```bash
    FC_VM=firecracker-vm

    gcloud compute instances create ${FC_VM} \
      --zone ${FC_ZONE} \
      --image ${FC_IMAGE}
    ```

4. Connect to the VM via SSH.

    ```bash
    gcloud compute ssh ${FC_VM}
    ```

5. Verify that VMX is enabled, enable KVM

    ```bash
    $ grep -cw vmx /proc/cpuinfo
    1
    $ sudo setfacl -m u:${USER}:rw /dev/kvm
    $ [ -r /dev/kvm ] && [ -w /dev/kvm ] && echo "OK" || echo "FAIL"
    OK
    ```
