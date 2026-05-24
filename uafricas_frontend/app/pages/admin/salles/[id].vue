<script setup lang="ts">
import type { GroupeEthniqueOption } from '~/composables/useAdminSalles'
import type { AdminPays } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { salleDetail, loading, error, chargerDetail, modifier, chargerGroupesEthniques } = useAdminSalles()
const { ajouterPaysOrigine, retirerPaysOrigine } = useAdminAfrolangSalles()
const { pays: paysListe, chargerListe: chargerPaysListe, filtres: filtresPays } = useAdminPays()

const ongletActif = ref('infos')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Modération admin (feature 001-ressources-fermeture-session, US2)
const modalReactivation = ref(false)
const rechargerSalleApresReactivation = async () => {
  await chargerDetail(id)
  successMsg.value = 'Salle réactivée avec succès'
  setTimeout(() => { successMsg.value = null }, 3000)
}

const form = reactive({
  titre: '',
  description: '',
  groupe_ethnique_id: '',
  langue_cible: '',
  langue_code: '',
  alphabet: '',
  dictionnaire_url: '',
  actif: true,
})

const groupesEthniques = ref<GroupeEthniqueOption[]>([])
const chargementGroupes = ref(false)

const chargerGroupes = async () => {
  chargementGroupes.value = true
  try {
    groupesEthniques.value = await chargerGroupesEthniques()
  } catch {
    // géré plus haut via error
  } finally {
    chargementGroupes.value = false
  }
}

const charger = async () => {
  await chargerDetail(id)
  if (salleDetail.value) {
    const s = salleDetail.value
    form.titre = s.titre
    form.description = s.description || ''
    form.groupe_ethnique_id = s.groupe_ethnique_id
    form.langue_cible = s.langue_cible || ''
    form.langue_code = s.langue_code || ''
    form.alphabet = s.alphabet || ''
    form.dictionnaire_url = s.dictionnaire_url || ''
    form.actif = s.actif
  }
}

const sauvegarderInfos = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: Record<string, any> = {}
    if (form.titre.trim()) body.titre = form.titre.trim()
    if (form.description.trim()) body.description = form.description.trim()
    if (form.groupe_ethnique_id && form.groupe_ethnique_id !== salleDetail.value?.groupe_ethnique_id) {
      body.groupe_ethnique_id = form.groupe_ethnique_id
    }
    body.langue_cible = form.langue_cible.trim()
    body.langue_code = form.langue_code.trim()
    body.alphabet = form.alphabet.trim()
    body.dictionnaire_url = form.dictionnaire_url.trim()
    body.actif = form.actif
    await modifier(id, body)
    successMsg.value = 'Salle mise à jour'
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

// Pays d'origine (feature 001-afrolang-pays-origine)
const paysSelectionne = ref<string>('')
const sauvegardePays = ref(false)
const erreurPays = ref<string | null>(null)

const paysDisponibles = computed<AdminPays[]>(() => {
  const dejaAssocies = new Set((salleDetail.value?.pays_origine ?? []).map(p => p.id))
  return paysListe.value.filter(p => p.actif && !dejaAssocies.has(p.id))
})

const ajouterPays = async () => {
  if (!paysSelectionne.value) return
  sauvegardePays.value = true
  erreurPays.value = null
  try {
    const ok = await ajouterPaysOrigine(id, paysSelectionne.value)
    if (!ok) throw new Error('Échec de l\'ajout du pays')
    paysSelectionne.value = ''
    await chargerDetail(id)
  } catch (e: any) {
    erreurPays.value = e?.data?.error || e?.message || 'Erreur lors de l\'ajout du pays'
  } finally {
    sauvegardePays.value = false
  }
}

const retirerPays = async (paysId: string) => {
  sauvegardePays.value = true
  erreurPays.value = null
  try {
    const ok = await retirerPaysOrigine(id, paysId)
    if (!ok) throw new Error('Échec du retrait du pays')
    await chargerDetail(id)
  } catch (e: any) {
    erreurPays.value = e?.data?.error || e?.message || 'Erreur lors du retrait du pays'
  } finally {
    sauvegardePays.value = false
  }
}

// Sessions de cette salle (via composable sessions)
const { sessions: sessionsListe, chargerListe: chargerSessions } = useAdminSessions()

const chargerSessionsSalle = async () => {
  const { filtres } = useAdminSessions()
  filtres.salle_id = id
  await chargerSessions()
}

watch(ongletActif, (val) => {
  if (val === 'sessions' && sessionsListe.value.length === 0) {
    chargerSessionsSalle()
  }
})

onMounted(() => {
  charger()
  chargerGroupes()
  // Charger la liste complète des pays (référentiel ~54 entrées) pour le sélecteur
  filtresPays.recherche = ''
  chargerPaysListe()
})
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="salleDetail?.titre || 'Chargement...'"
      sous-titre="Édition de la salle AfroLang"
    >
      <template #actions>
        <NuxtLink to="/admin/salles" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !salleDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="salleDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-primary text-primary-content rounded-full w-16 h-16">
            <span class="text-xl"><font-awesome-icon icon="video" /></span>
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ salleDetail.titre }}</h2>
          <p class="text-sm text-base-content/60">
            {{ salleDetail.langue_cible || 'Langue non définie' }}
            {{ salleDetail.groupe_ethnique_nom ? ` — ${salleDetail.groupe_ethnique_nom}` : '' }}
          </p>
          <div class="flex gap-2 mt-1">
            <span :class="salleDetail.actif ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
              {{ salleDetail.actif ? 'Active' : 'Inactive' }}
            </span>
            <span class="badge badge-outline badge-sm">{{ salleDetail.nombre_salles_privees }} salles privées</span>
            <span class="badge badge-outline badge-sm">{{ salleDetail.nombre_sessions }} sessions</span>
            <span class="badge badge-outline badge-sm">{{ salleDetail.nombre_moderateurs_attitres }} modérateurs attitrés</span>
          </div>
        </div>
      </div>

      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">
          <font-awesome-icon icon="xmark" />
        </button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Onglets -->
      <div role="tablist" class="tabs tabs-bordered mb-6">
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="circle-info" class="mr-1" /> Infos
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'sessions' }" @click="ongletActif = 'sessions'">
          <font-awesome-icon icon="video" class="mr-1" /> Sessions
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'pays' }" @click="ongletActif = 'pays'">
          <font-awesome-icon icon="globe" class="mr-1" /> Pays d'origine
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'administrateurs' }" @click="ongletActif = 'administrateurs'">
          <font-awesome-icon icon="user-shield" class="mr-1" /> Administrateurs de salle
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'historique-moderation' }" @click="ongletActif = 'historique-moderation'">
          <font-awesome-icon icon="gavel" class="mr-1" /> Historique de modération
        </button>
      </div>

      <!-- Bandeau désactivation administrative (feature 001-ressources-fermeture-session, US2) -->
      <div
        v-if="salleDetail.desactivee_admin"
        class="alert alert-error mb-4 flex items-center justify-between"
      >
        <div>
          <font-awesome-icon icon="ban" class="mr-2" />
          <span class="font-semibold">Salle désactivée par administration</span>
          <p v-if="salleDetail.desactivee_admin.motif" class="text-sm mt-1">
            Motif : {{ salleDetail.desactivee_admin.motif }}
          </p>
          <p class="text-xs opacity-75">
            Désactivée le {{ new Date(salleDetail.desactivee_admin.desactivee_at).toLocaleString('fr-FR') }}
          </p>
        </div>
        <button class="btn btn-success btn-sm" @click="modalReactivation = true">
          <font-awesome-icon icon="circle-check" class="mr-1" />
          Réactiver
        </button>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarderInfos" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Titre de la salle *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Groupe ethnique *</span></label>
              <select
                v-model="form.groupe_ethnique_id"
                class="select select-bordered"
                :disabled="chargementGroupes"
                required
              >
                <option value="" disabled>Sélectionner un groupe ethnique</option>
                <option
                  v-for="g in groupesEthniques"
                  :key="g.id"
                  :value="g.id"
                  :disabled="g.salle_active && g.id !== salleDetail.groupe_ethnique_id"
                >
                  {{ g.nom }}{{ g.pays_nom ? ` — ${g.pays_nom}` : '' }}{{ g.salle_active && g.id !== salleDetail.groupe_ethnique_id ? ' (déjà une salle active)' : '' }}
                </option>
              </select>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Langue cible</span></label>
                <input v-model="form.langue_cible" type="text" class="input input-bordered" placeholder="Ex: Swahili, Wolof...">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Code langue</span></label>
                <input v-model="form.langue_code" type="text" class="input input-bordered" placeholder="Ex: sw, wo...">
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Alphabet</span></label>
              <textarea
                v-model="form.alphabet"
                class="textarea textarea-bordered"
                rows="2"
                placeholder="Caractères de l'alphabet (affichés dans l'onglet Ressources)"
              />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">URL dictionnaire</span></label>
              <input
                v-model="form.dictionnaire_url"
                type="url"
                class="input input-bordered"
                placeholder="https://..."
              >
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.actif" type="checkbox" class="toggle toggle-success" />
                <span class="label-text">Salle active</span>
              </label>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                ID: {{ salleDetail.id.substring(0, 8) }}... | Slug: {{ salleDetail.slug }}
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Pays d'origine (feature 001-afrolang-pays-origine) -->
      <div v-if="ongletActif === 'pays'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">
            Pays d'origine de la langue
            <span class="badge badge-outline badge-sm ml-2">{{ salleDetail.pays_origine?.length ?? 0 }}</span>
          </h3>

          <p class="text-sm text-base-content/60 mb-4">
            Associe à cette salle les pays où la langue cible est parlée à l'origine. Visible côté public sur la carte de la salle. Les pays archivés sont affichés en gris et peuvent être retirés.
          </p>

          <div v-if="erreurPays" class="alert alert-error mb-4">
            <font-awesome-icon icon="circle-exclamation" />
            <span>{{ erreurPays }}</span>
          </div>

          <!-- Liste des pays associés -->
          <div class="flex flex-wrap gap-2 mb-6 min-h-[2.5rem]">
            <span
              v-for="p in salleDetail.pays_origine ?? []"
              :key="p.id"
              class="badge badge-lg gap-2"
              :class="paysListe.find(pl => pl.id === p.id && !pl.actif) ? 'badge-ghost opacity-60' : 'badge-primary'"
              :title="paysListe.find(pl => pl.id === p.id && !pl.actif) ? 'Pays archivé — masqué côté public' : ''"
            >
              <span v-if="p.code_iso2">{{ p.code_iso2 }}</span>
              <span>{{ p.nom }}</span>
              <span v-if="paysListe.find(pl => pl.id === p.id && !pl.actif)" class="text-[10px]">(archivé)</span>
              <button
                class="btn btn-ghost btn-xs btn-circle"
                :disabled="sauvegardePays"
                @click="retirerPays(p.id)"
                aria-label="Retirer ce pays"
              >
                <font-awesome-icon icon="xmark" />
              </button>
            </span>
            <span v-if="!salleDetail.pays_origine?.length" class="text-sm text-base-content/50 italic">
              Aucun pays d'origine associé
            </span>
          </div>

          <!-- Sélecteur d'ajout -->
          <div class="flex items-end gap-2">
            <div class="form-control flex-1">
              <label class="label"><span class="label-text">Ajouter un pays</span></label>
              <select v-model="paysSelectionne" class="select select-bordered" :disabled="sauvegardePays">
                <option value="" disabled>Sélectionner un pays</option>
                <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">
                  {{ p.code_iso2 ? `${p.code_iso2} — ` : '' }}{{ p.nom }}
                </option>
              </select>
            </div>
            <button
              class="btn btn-primary"
              :disabled="!paysSelectionne || sauvegardePays"
              :class="{ loading: sauvegardePays }"
              @click="ajouterPays"
            >
              <font-awesome-icon v-if="!sauvegardePays" icon="plus" class="mr-1" />
              Ajouter
            </button>
          </div>
        </div>
      </div>

      <!-- Onglet Administrateurs de salle (feature 001-admin-salles-publiques, US3) -->
      <div v-if="ongletActif === 'administrateurs'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-2">
            Administrateurs de cette salle publique
          </h3>
          <p class="text-sm text-base-content/60 mb-4">
            Distinct du panneau « Modérateurs attitrés » et du rôle global « Admin plateforme ».
            Les capacités effectives seront ajoutées progressivement (FR-019).
          </p>
          <AdminAfrolangSalleAdministrateursPanel :salle-id="id" />
        </div>
      </div>

      <!-- Onglet Historique de modération (feature 001-ressources-fermeture-session, US3) -->
      <div v-if="ongletActif === 'historique-moderation'">
        <AdminAfrolangSalleHistoriqueModerationPanel :salle-id="id" />
      </div>

      <!-- Modal de réactivation (feature 001-ressources-fermeture-session, US2) -->
      <AdminAfrolangSalleReactivationModal
        :open="modalReactivation"
        :salle-id="id"
        :salle-titre="salleDetail?.titre"
        @close="modalReactivation = false"
        @success="rechargerSalleApresReactivation"
      />

      <!-- Onglet Sessions -->
      <div v-if="ongletActif === 'sessions'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Sessions de cette salle</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Titre</th>
                <th class="w-24">État</th>
                <th class="w-36">Date</th>
                <th class="w-24 text-center">Participants</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="session in sessionsListe" :key="session.id">
                <td>{{ session.titre || 'Sans titre' }}</td>
                <td>
                  <span class="badge badge-sm" :class="{
                    'badge-info': session.etat === 'planifiee',
                    'badge-success': session.etat === 'en_cours',
                    'badge-neutral': session.etat === 'terminee',
                    'badge-error': session.etat === 'annulee',
                  }">
                    {{ session.etat || '—' }}
                  </span>
                </td>
                <td>{{ session.demarre_at ? new Date(session.demarre_at).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' }) : '—' }}</td>
                <td class="text-center">{{ session.nombre_participants_pic ?? '—' }}</td>
                <td>
                  <NuxtLink :to="`/admin/sessions/${session.id}`" class="btn btn-ghost btn-xs">
                    <font-awesome-icon icon="eye" />
                  </NuxtLink>
                </td>
              </tr>
              <tr v-if="!sessionsListe.length">
                <td colspan="5" class="text-center text-base-content/50 py-4">Aucune session</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>
