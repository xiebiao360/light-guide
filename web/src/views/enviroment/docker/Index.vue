<script setup>
import { ref } from 'vue'
import { getVersion, getPackages, installPackage } from '@/api'
import { message } from 'ant-design-vue'
import { UploadOutlined } from '@ant-design/icons-vue'
import { format_timestamp } from '@/utils'

getVersion().then(res => {
  version.value = res.data.version
})

const load_packages = () => {
  getPackages().then(res => {
    res.data.forEach(item => {
      dataSource.push({
        key: item.index,
        name: item.name,
        time: format_timestamp(item.create_time),
      })
    })
  })
}

load_packages()

const version = ref('')

const fileList = ref([])

const spinning = ref(false)

const beforeUpload = file => {
  // const isPng = file.type === 'image/png'
  // if (!isPng) {
  //   message.error('You can only upload PNG file!')
  // }
  // return isPng

  console.log('file type: ' + file.type)
  return true
}

const handleChange = info => {
  if (info.file.status !== 'uploading') {
    console.log(info.file, info.fileList)
    spinning.value = true
  }
  if (info.file.status === 'done') {
    message.success(`${info.file.name} file uploaded successfully`)
    spinning.value = false

    load_packages()
  } else if (info.file.status === 'error') {
    message.error(`${info.file.name} file upload failed.`)
    spinning.value = false
  }
}

const columns = [
  {
    title: '文件名',
    dataIndex: 'name',
    key: 'name',
  },
  {
    title: '上传时间',
    dataIndex: 'time',
    key: 'time',
  },
  {
    title: '操作',
    key: 'action',
  },
]
const dataSource = []

const install_package = name => {
  installPackage(name).then(res => {
    message.success('安装成功')
  })
}
</script>
<template>
  <a-space direction="vertical" style="width: 100%">
    <div>
      <template v-if="version !== ''">
        <a-alert message="Docker 已安装" :description="'Docker version: ' + version" type="success" />
      </template>
      <template v-else>
        <a-alert message="Docker 未安装" description="点击操作按钮立即安装" type="error" closable />
      </template>
    </div>
    <template v-if="version != ''">
      <a-spin :spinning="spinning">
        <a-space>
          <a-upload v-model:file-list="fileList" action="/api/docker/upload" :showUploadList="false" :before-upload="beforeUpload" @change="handleChange">
            <a-button type="primary">
              <upload-outlined></upload-outlined>
              导入安装包
            </a-button>
          </a-upload>
          <a-button type="primary">下载安装包</a-button>
        </a-space>
        <a-table :dataSource="dataSource" :columns="columns">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'action'">
              <a-space>
                <a-button type="primary" @click="install_package(record.name)">安装</a-button>
                <a-button type="primary">删除</a-button>
              </a-space>
            </template>
          </template>
        </a-table>
      </a-spin>
    </template>
  </a-space>
</template>
