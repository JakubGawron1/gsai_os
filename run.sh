#!/bin/bash

qemu-system-x86_64 \
    -no-reboot \
    -machine q35 \
    -cpu qemu64 \
    -smp 1 \
    -m 64M \
    -serial stdio \
    -display none \
    -net none \
    -bios ./ovmf.fd \
    -drive format=raw,file=fat:rw:./.hdd/image/ \
    -drive format=raw,file=./.hdd/nvme.img,id=nvm,if=none \
    -device nvme,drive=nvm,serial=deadbeef \
    -D .debug/qemu_debug.log \
    -d int,guest_errors,trace:nvme_controller_capability_raw,trace:nvme_controller_capability,trace:nvme_controller_spec_version,trace:nvme_kick,trace:nvme_dma_flush_queue_wait,trace:nvme_error,trace:nvme_process_completion,trace:nvme_process_completion_queue_plugged,trace:nvme_complete_command,trace:nvme_submit_command,trace:nvme_submit_command_raw,trace:nvme_handle_event,trace:nvme_poll_queue,trace:nvme_prw_aligned,trace:nvme_write_zeroes,trace:nvme_qiov_unaligned,trace:nvme_prw_buffered,trace:nvme_rw_done,trace:nvme_dsm,trace:nvme_dsm_done,trace:nvme_dma_map_flush,trace:nvme_free_req_queue_wait,trace:nvme_create_queue_pair,trace:nvme_free_queue_pair,trace:nvme_cmd_map_qiov,trace:nvme_cmd_map_qiov_pages,trace:nvme_cmd_map_qiov_iov,trace:pci_nvme_irq_msix,trace:pci_nvme_irq_pin,trace:pci_nvme_irq_masked,trace:pci_nvme_dma_read,trace:pci_nvme_map_addr,trace:pci_nvme_map_addr_cmb,trace:pci_nvme_map_prp,trace:pci_nvme_map_sgl,trace:pci_nvme_io_cmd,trace:pci_nvme_admin_cmd,trace:pci_nvme_create_sq,trace:pci_nvme_create_cq,trace:pci_nvme_del_sq,trace:pci_nvme_del_cq,trace:pci_nvme_identify_ns_descr_list,trace:pci_nvme_get_log,trace:pci_nvme_getfeat,trace:pci_nvme_setfeat,trace:pci_nvme_getfeat_vwcache,trace:pci_nvme_getfeat_numq,trace:pci_nvme_setfeat_numq,trace:pci_nvme_setfeat_timestamp,trace:pci_nvme_getfeat_timestamp,trace:pci_nvme_process_aers,trace:pci_nvme_aer,trace:pci_nvme_aer_aerl_exceeded,trace:pci_nvme_aer_masked,trace:pci_nvme_aer_post_cqe,trace:pci_nvme_enqueue_event,trace:pci_nvme_enqueue_event_noqueue,trace:pci_nvme_enqueue_event_masked,trace:pci_nvme_no_outstanding_aers,trace:pci_nvme_enqueue_req_completion,trace:pci_nvme_mmio_read,trace:pci_nvme_mmio_write,trace:pci_nvme_mmio_doorbell_cq,trace:pci_nvme_mmio_doorbell_sq,trace:pci_nvme_mmio_intm_set,trace:pci_nvme_mmio_intm_clr,trace:pci_nvme_mmio_cfg,trace:pci_nvme_mmio_aqattr,trace:pci_nvme_mmio_asqaddr,trace:pci_nvme_mmio_acqaddr,trace:pci_nvme_mmio_asqaddr_hi,trace:pci_nvme_mmio_acqaddr_hi,trace:pci_nvme_mmio_start_success,trace:pci_nvme_mmio_stopped,trace:pci_nvme_mmio_shutdown_set,trace:pci_nvme_mmio_shutdown_cleared,trace:pci_nvme_err_mdts,trace:pci_nvme_err_req_status,trace:pci_nvme_err_addr_read,trace:pci_nvme_err_addr_write,trace:pci_nvme_err_cfs,trace:pci_nvme_err_aio,trace:pci_nvme_err_invalid_sgld,trace:pci_nvme_err_invalid_num_sgld,trace:pci_nvme_err_invalid_sgl_excess_length,trace:pci_nvme_err_invalid_dma,trace:pci_nvme_err_invalid_prplist_ent,trace:pci_nvme_err_invalid_prp2_align,trace:pci_nvme_err_invalid_opc,trace:pci_nvme_err_invalid_admin_opc,trace:pci_nvme_err_invalid_lba_range,trace:pci_nvme_err_unaligned_zone_cmd,trace:pci_nvme_err_invalid_zone_state_transition,trace:pci_nvme_err_write_not_at_wp,trace:pci_nvme_err_append_not_at_start,trace:pci_nvme_err_zone_is_full,trace:pci_nvme_err_zone_is_read_only,trace:pci_nvme_err_zone_is_offline,trace:pci_nvme_err_zone_boundary,trace:pci_nvme_err_zone_invalid_write,trace:pci_nvme_err_zone_write_not_ok,trace:pci_nvme_err_zone_read_not_ok,trace:pci_nvme_err_insuff_active_res,trace:pci_nvme_err_insuff_open_res,trace:pci_nvme_err_zd_extension_map_error,trace:pci_nvme_err_invalid_iocsci,trace:pci_nvme_err_invalid_del_sq,trace:pci_nvme_err_invalid_create_sq_cqid,trace:pci_nvme_err_invalid_create_sq_sqid,trace:pci_nvme_err_invalid_create_sq_size,trace:pci_nvme_err_invalid_create_sq_addr,trace:pci_nvme_err_invalid_create_sq_qflags,trace:pci_nvme_err_invalid_del_cq_cqid,trace:pci_nvme_err_invalid_del_cq_notempty,trace:pci_nvme_err_invalid_create_cq_cqid,trace:pci_nvme_err_invalid_create_cq_size,trace:pci_nvme_err_invalid_create_cq_addr,trace:pci_nvme_err_invalid_create_cq_vector,trace:pci_nvme_err_invalid_create_cq_qflags,trace:pci_nvme_err_invalid_identify_cns,trace:pci_nvme_err_invalid_getfeat,trace:pci_nvme_err_invalid_setfeat,trace:pci_nvme_err_invalid_log_page,trace:pci_nvme_err_startfail_cq,trace:pci_nvme_err_startfail_sq,trace:pci_nvme_err_startfail_asq_misaligned,trace:pci_nvme_err_startfail_acq_misaligned,trace:pci_nvme_err_startfail_page_too_small,trace:pci_nvme_err_startfail_page_too_large,trace:pci_nvme_err_startfail_cqent_too_small,trace:pci_nvme_err_startfail_cqent_too_large,trace:pci_nvme_err_startfail_sqent_too_small,trace:pci_nvme_err_startfail_sqent_too_large,trace:pci_nvme_err_startfail_css,trace:pci_nvme_err_startfail_asqent_sz_zero,trace:pci_nvme_err_startfail_acqent_sz_zero,trace:pci_nvme_err_startfail_zasl_too_small,trace:pci_nvme_err_startfail,trace:pci_nvme_err_invalid_mgmt_action,trace:pci_nvme_ub_mmiowr_misaligned32,trace:pci_nvme_ub_mmiowr_toosmall,trace:pci_nvme_ub_mmiowr_intmask_with_msix,trace:pci_nvme_ub_mmiowr_ro_csts,trace:pci_nvme_ub_mmiowr_ssreset_w1c_unsupported,trace:pci_nvme_ub_mmiowr_ssreset_unsupported,trace:pci_nvme_ub_mmiowr_cmbloc_reserved,trace:pci_nvme_ub_mmiowr_cmbsz_readonly,trace:pci_nvme_ub_mmiowr_pmrcap_readonly,trace:pci_nvme_ub_mmiowr_pmrsts_readonly,trace:pci_nvme_ub_mmiowr_pmrebs_readonly,trace:pci_nvme_ub_mmiowr_pmrswtp_readonly,trace:pci_nvme_ub_mmiowr_invalid,trace:pci_nvme_ub_mmiord_misaligned32,trace:pci_nvme_ub_mmiord_toosmall,trace:pci_nvme_ub_mmiord_invalid_ofs,trace:pci_nvme_ub_db_wr_misaligned,trace:pci_nvme_ub_db_wr_invalid_cq,trace:pci_nvme_ub_db_wr_invalid_cqhead,trace:pci_nvme_ub_db_wr_invalid_sq,trace:pci_nvme_ub_db_wr_invalid_sqtail,trace:pci_nvme_ub_unknown_css_value,trace:pci_nvme_ub_too_many_mappings