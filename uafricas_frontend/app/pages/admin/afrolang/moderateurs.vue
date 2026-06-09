<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

interface SalleAdminListe {
  id: string
  titre: string
  slug: string | null
  langue_cible: string | null
  groupe_ethnique_nom: string | null
  actif: boolean
  nombre_moderateurs_attitres: number
}

definePageMeta({
  layout: 'admin',
  middleware: ['admin'],
})

useHead({
  title: 'Afrolang — Modérateurs attitrés | Admin',
})

const { adminFetch } = useAdmin()

const salles = ref<SalleAdminListe[]>([])
const recherche = ref('')
const salleSelectionneeId = ref<string | null>(null)
const chargement = ref(false)

const chargerSalles = async () => {
  chargement.value = true
  try {
    const resp = await adminFetch<{
      success: boolean
      // Réponse paginée standard : les éléments sont sous `data.data`
      // (PaginatedResponse), pas `data.items`.
      data?: { data: SalleAdminListe[] }
    }>('/api/admin/salles', {
      params: {
        par_page: 100,
        recherche: recherche.value.trim() || undefined,
      },
    })
    salles.value = resp?.data?.data ?? []
  } finally {
    chargement.value = false
  }
}

const sallesFiltrees = computed(() => salles.value)

onMounted(() => chargerSalles())
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Afrolang — Modérateurs attitrés"
      description="Désignez les modérateurs Afrolang pour chaque salle publique (affectation many-to-many, stable)."
      icone="user-shield"
    />

    <div class="grid grid-cols-1 lg:grid-cols-[280px_1fr] gap-4 mt-4">
      <aside class="card bg-base-100 shadow">
        <div class="card-body p-3">
          <div class="form-control mb-2">
            <input
              v-model="recherche"
              type="text"
              placeholder="Rechercher une salle…"
              class="input input-bordered input-sm"
              @keyup.enter="chargerSalles"
            />
          </div>
          <div v-if="chargement" class="text-center py-4">
            <span class="loading loading-spinner loading-sm" />
          </div>
          <ul v-else class="menu menu-sm max-h-[70vh] overflow-y-auto">
            <li v-for="s in sallesFiltrees" :key="s.id">
              <button
                type="button"
                class="flex flex-col items-start text-left"
                :class="{ active: salleSelectionneeId === s.id }"
                @click="salleSelectionneeId = s.id"
              >
                <span class="font-semibold">{{ s.titre }}</span>
                <span class="text-xs opacity-70">
                  {{ s.groupe_ethnique_nom || '—' }} ·
                  {{ s.nombre_moderateurs_attitres }} mod.
                </span>
              </button>
            </li>
          </ul>
        </div>
      </aside>

      <section class="card bg-base-100 shadow">
        <div class="card-body">
          <AdminAfrolangModerateursAttitresPanel :salle-id="salleSelectionneeId" />
        </div>
      </section>
    </div>
  </div>
</template>
